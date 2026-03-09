//! Audio processing engine for FXSound.
//!
//! Provides a 10-band equalizer using biquad peak filters, audio effects
//! (fidelity, dynamic compression, bass boost), and real-time FFT-based
//! spectrum analysis for the visualizer.

use std::collections::HashMap;
use std::sync::Arc;
use libpulse_binding as pulse;
use libpulse_simple_binding as psimple;
use rustfft::{FftPlanner, num_complex::Complex};

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: u8 = 2;
const FFT_SIZE: usize = 512;

/// Center frequencies for the 10 EQ bands (Hz).
const EQ_FREQUENCIES: [f32; 10] = [
    32.0, 64.0, 125.0, 250.0, 500.0,
    1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

// ──────────────────────────────────────────────
//  Biquad Filter
// ──────────────────────────────────────────────

/// Second-order IIR (biquad) filter coefficients and state.
///
/// Used for peaking EQ filters — each band gets its own biquad
/// that only boosts/cuts around its center frequency.
#[derive(Clone)]
struct BiquadFilter {
    // Coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // Delay line (filter state for two channels)
    x1: [f32; 2],
    x2: [f32; 2],
    y1: [f32; 2],
    y2: [f32; 2],
}

impl BiquadFilter {
    /// Create a peaking EQ filter.
    ///
    /// - `freq` — center frequency in Hz
    /// - `gain_db` — boost/cut in dB (positive = boost, negative = cut)
    /// - `q` — quality factor (higher = narrower band)
    /// - `sample_rate` — audio sample rate in Hz
    fn peaking_eq(freq: f32, gain_db: f32, q: f32, sample_rate: f32) -> Self {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let alpha = w0.sin() / (2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * w0.cos();
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * w0.cos();
        let a2 = 1.0 - alpha / a;

        // Normalize by a0
        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Create a flat (pass-through) filter — all coefficients set for unity gain.
    fn flat() -> Self {
        Self {
            b0: 1.0, b1: 0.0, b2: 0.0,
            a1: 0.0, a2: 0.0,
            x1: [0.0; 2], x2: [0.0; 2],
            y1: [0.0; 2], y2: [0.0; 2],
        }
    }

    /// Process a single sample through the filter for the given channel.
    fn process(&mut self, input: f32, channel: usize) -> f32 {
        let ch = channel % 2;
        let output = self.b0 * input
            + self.b1 * self.x1[ch]
            + self.b2 * self.x2[ch]
            - self.a1 * self.y1[ch]
            - self.a2 * self.y2[ch];

        // Shift delay line
        self.x2[ch] = self.x1[ch];
        self.x1[ch] = input;
        self.y2[ch] = self.y1[ch];
        self.y1[ch] = output;

        output
    }
}

// ──────────────────────────────────────────────
//  Audio Engine
// ──────────────────────────────────────────────

/// Core audio processing state.
///
/// Holds the EQ band gains, effect values, biquad filter instances,
/// and shared FFT data for the visualizer.
pub struct AudioEngine {
    powered: bool,
    eq_bands: [f32; 10],
    effects: HashMap<String, f32>,
    sample_rate: u32,

    /// One biquad filter per EQ band — rebuilt when gain changes.
    filters: Vec<BiquadFilter>,

    /// FFT magnitude data shared with the UI for the visualizer.
    pub fft_data: Arc<std::sync::Mutex<Vec<f32>>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        // Start with flat (0 dB) filters for all 10 bands
        let filters = EQ_FREQUENCIES
            .iter()
            .map(|_| BiquadFilter::flat())
            .collect();

        Self {
            powered: true,
            eq_bands: [0.0; 10],
            effects: HashMap::new(),
            sample_rate: SAMPLE_RATE,
            filters,
            fft_data: Arc::new(std::sync::Mutex::new(vec![0.0; 32])),
        }
    }

    /// Set the gain for a single EQ band and rebuild its biquad filter.
    pub fn set_eq_band(&mut self, band: usize, gain: f32) {
        if band >= 10 {
            return;
        }
        self.eq_bands[band] = gain.clamp(-12.0, 12.0);

        // Rebuild the biquad filter for this band with the new gain
        // Q factor of 1.4 gives a moderate bandwidth suitable for a 10-band EQ
        if self.eq_bands[band].abs() < 0.1 {
            self.filters[band] = BiquadFilter::flat();
        } else {
            self.filters[band] = BiquadFilter::peaking_eq(
                EQ_FREQUENCIES[band],
                self.eq_bands[band],
                1.4,
                self.sample_rate as f32,
            );
        }
        log::info!("EQ band {} set to {:.1} dB", band, self.eq_bands[band]);
    }

    /// Set an effect intensity value (0–100).
    pub fn set_effect(&mut self, effect: &str, value: f32) {
        let clamped = value.clamp(0.0, 100.0);
        self.effects.insert(effect.to_string(), clamped);
        log::info!("Effect '{}' set to {:.1}", effect, clamped);
    }

    /// Toggle audio processing on or off.
    /// When off, the audio loop outputs silence rather than passthrough
    /// to avoid doubling the original audio.
    pub fn set_power(&mut self, enabled: bool) {
        self.powered = enabled;
        log::info!("Power: {}", if enabled { "ON" } else { "OFF" });
    }

    /// Return the current FFT magnitude data for the visualizer (32 bins).
    pub fn get_fft_data(&self) -> Vec<f32> {
        self.fft_data.lock().unwrap().clone()
    }

    // ── Main processing pipeline ──

    /// Process an audio buffer: apply EQ, effects, limiter, and update the visualizer.
    ///
    /// When powered off, the output is filled with silence to prevent
    /// audio doubling (the original system audio is already playing).
    pub fn process_audio(&mut self, input: &[f32], output: &mut [f32]) {
        if !self.powered {
            output.fill(0.0);
            return;
        }

        // Skip near-silent input to avoid amplifying noise
        let rms: f32 = input.iter().map(|&x| x * x).sum::<f32>() / input.len() as f32;
        if rms.sqrt() < 0.001 {
            output.fill(0.0);
            return;
        }

        // Apply the 10-band EQ using biquad filters
        self.apply_eq(input, output);

        // Apply audio effects (fidelity, dynamic compression, bass boost)
        self.apply_effects(output);

        // Hard limiter — prevent clipping
        self.apply_limiter(output);

        // Update FFT data for the visualizer
        self.update_fft(output);
    }

    // ── EQ Processing ──

    /// Apply all 10 biquad EQ filters in series to the audio buffer.
    ///
    /// Each filter only affects frequencies around its center frequency,
    /// so adjusting the 32 Hz band won't change treble, and vice versa.
    fn apply_eq(&mut self, input: &[f32], output: &mut [f32]) {
        output.copy_from_slice(input);

        for band in 0..10 {
            // Skip flat bands for efficiency
            if self.eq_bands[band].abs() < 0.1 {
                continue;
            }

            // Process each sample through this band's biquad filter
            // Interleaved stereo: even indices = left, odd = right
            for (i, sample) in output.iter_mut().enumerate() {
                let channel = i % (CHANNELS as usize);
                *sample = self.filters[band].process(*sample, channel);
            }
        }
    }

    // ── Effects Processing ──

    /// Apply audio effects to the buffer.
    fn apply_effects(&self, buffer: &mut [f32]) {
        // Fidelity: subtle high-frequency harmonic enhancement
        if let Some(&fidelity) = self.effects.get("fidelity") {
            if fidelity > 0.0 {
                let amount = fidelity / 100.0;
                for sample in buffer.iter_mut() {
                    // Soft saturation — adds harmonics that brighten the sound
                    let saturated = (sample.abs() * (1.0 + amount * 0.5)).tanh() * sample.signum();
                    *sample = *sample * (1.0 - amount * 0.3) + saturated * (amount * 0.3);
                }
            }
        }

        // Dynamic compression: reduces the gap between loud and quiet
        if let Some(&dynamic) = self.effects.get("dynamic") {
            if dynamic > 0.0 {
                let threshold = 0.7 - (dynamic / 100.0) * 0.3;
                let ratio = 0.5 + (1.0 - dynamic / 100.0) * 0.5; // 2:1 at max
                for sample in buffer.iter_mut() {
                    if sample.abs() > threshold {
                        let sign = sample.signum();
                        let excess = sample.abs() - threshold;
                        *sample = sign * (threshold + excess * ratio);
                    }
                }
            }
        }

        // Bass boost: apply gain to low frequencies
        // (simplified — applies a uniform boost; proper version would use a low-shelf filter)
        if let Some(&bass) = self.effects.get("bass") {
            if bass > 0.0 {
                let boost = 1.0 + (bass / 100.0) * 0.3;
                for sample in buffer.iter_mut() {
                    *sample *= boost;
                }
            }
        }
    }

    // ── Limiter ──

    /// Prevent clipping by normalizing if any sample exceeds ±1.0.
    fn apply_limiter(&self, buffer: &mut [f32]) {
        let peak = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        if peak > 1.0 {
            let scale = 1.0 / peak;
            for sample in buffer.iter_mut() {
                *sample *= scale;
            }
        }
    }

    // ── Visualizer FFT ──

    /// Compute FFT magnitudes from the output buffer and store for the visualizer.
    fn update_fft(&self, buffer: &[f32]) {
        if buffer.len() < FFT_SIZE {
            return;
        }

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);

        let mut complex_input: Vec<Complex<f32>> = buffer[..FFT_SIZE]
            .iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();

        fft.process(&mut complex_input);

        // Convert to magnitudes, scaled for display (0–100 range)
        let magnitudes: Vec<f32> = complex_input[..32]
            .iter()
            .map(|c| (c.norm() * 100.0).min(100.0))
            .collect();

        if let Ok(mut fft_data) = self.fft_data.lock() {
            *fft_data = magnitudes;
        }
    }
}

/// Convert decibels to linear gain.
fn _db_to_linear(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}

// ──────────────────────────────────────────────
//  PulseAudio Integration
// ──────────────────────────────────────────────

/// Manages the PulseAudio capture/playback loop.
///
/// Captures system audio from the monitor source, runs it through
/// the AudioEngine for processing, and outputs the result.
pub struct AudioProcessor {
    engine: Arc<std::sync::Mutex<AudioEngine>>,
}

impl AudioProcessor {
    pub fn new(engine: Arc<std::sync::Mutex<AudioEngine>>) -> Self {
        Self { engine }
    }

    /// Start the audio processing loop in a background thread.
    pub fn start(&self) -> Result<(), String> {
        log::info!("Starting PipeWire/PulseAudio processor...");

        let spec = pulse::sample::Spec {
            format: pulse::sample::Format::F32le,
            channels: CHANNELS,
            rate: SAMPLE_RATE,
        };

        if !spec.is_valid() {
            return Err("Invalid audio spec".to_string());
        }

        let engine = Arc::clone(&self.engine);

        std::thread::spawn(move || {
            match Self::audio_loop(engine, spec) {
                Ok(_) => log::info!("Audio loop ended normally"),
                Err(e) => log::error!("Audio loop error: {}", e),
            }
        });

        Ok(())
    }

    /// Main audio capture → process → playback loop.
    ///
    /// Reads from the system monitor source (captures all desktop audio),
    /// processes it through the AudioEngine, and writes to an output stream.
    fn audio_loop(
        engine: Arc<std::sync::Mutex<AudioEngine>>,
        spec: pulse::sample::Spec,
    ) -> Result<(), String> {
        // Try to open the monitor source (captures system audio output)
        let input = psimple::Simple::new(
            None,
            "FXSound Input",
            pulse::stream::Direction::Record,
            Some("@DEFAULT_MONITOR@"),
            "Capture System Audio",
            &spec,
            None,
            None,
        ).map_err(|e| {
            log::warn!("Failed to open monitor source: {}. Trying default source...", e);
            e
        });

        let input = match input {
            Ok(stream) => stream,
            Err(_) => {
                // Fallback: use the default recording source
                psimple::Simple::new(
                    None,
                    "FXSound Input",
                    pulse::stream::Direction::Record,
                    None,
                    "Capture System Audio",
                    &spec,
                    None,
                    None,
                ).map_err(|e| format!("Failed to create input stream: {}", e))?
            }
        };

        // Create the playback output stream
        let output = psimple::Simple::new(
            None,
            "FXSound Output",
            pulse::stream::Direction::Playback,
            None,
            "Processed Audio",
            &spec,
            None,
            None,
        ).map_err(|e| format!("Failed to create output stream: {}", e))?;

        log::info!("Audio streams created successfully");
        log::info!("Processing system audio through FXSound...");

        const BUFFER_SIZE: usize = 1024;
        let mut input_bytes = vec![0u8; BUFFER_SIZE * 4]; // f32 = 4 bytes
        let mut output_samples = vec![0f32; BUFFER_SIZE];

        loop {
            // Read raw bytes from the input stream
            if let Err(e) = input.read(&mut input_bytes) {
                log::error!("Read error: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            // Convert bytes to f32 samples
            let input_samples: Vec<f32> = input_bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();

            // Process audio through the engine
            {
                let mut engine = engine.lock().unwrap();
                engine.process_audio(&input_samples, &mut output_samples);
            }

            // Convert f32 samples back to bytes and write to output
            let output_bytes: Vec<u8> = output_samples
                .iter()
                .flat_map(|&f| f.to_le_bytes())
                .collect();

            if let Err(e) = output.write(&output_bytes) {
                log::error!("Write error: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
        }
    }
}

// ──────────────────────────────────────────────
//  PulseAudio Device Detection
// ──────────────────────────────────────────────

/// Query PulseAudio for available audio output sinks.
///
/// Uses the introspection API to list all sinks and return their
/// human-readable descriptions (e.g. "Built-in Audio Analog Stereo").
pub fn get_pulse_sinks() -> Result<Vec<String>, String> {
    use pulse::context::{Context, FlagSet as ContextFlagSet};
    use pulse::mainloop::threaded::Mainloop;
    use std::sync::{Arc, Mutex, Condvar};
    use std::time::Duration;

    // Create a threaded mainloop for the introspection query
    let mainloop = Mainloop::new().ok_or("Failed to create PulseAudio mainloop")?;
    mainloop.start().map_err(|e| format!("Failed to start mainloop: {}", e))?;

    let context = Context::new(&mainloop, "FXSound Device Query")
        .ok_or("Failed to create PulseAudio context")?;

    // Lock the mainloop while connecting
    mainloop.lock();
    context
        .connect(None, ContextFlagSet::NOFLAGS, None)
        .map_err(|e| {
            mainloop.unlock();
            format!("Failed to connect context: {}", e)
        })?;

    // Wait for the context to be ready (up to 5 seconds)
    let start = std::time::Instant::now();
    loop {
        match context.get_state() {
            pulse::context::State::Ready => break,
            pulse::context::State::Failed | pulse::context::State::Terminated => {
                mainloop.unlock();
                mainloop.stop();
                return Err("PulseAudio context failed".to_string());
            }
            _ => {}
        }
        if start.elapsed() > Duration::from_secs(5) {
            mainloop.unlock();
            mainloop.stop();
            return Err("Timeout waiting for PulseAudio context".to_string());
        }
        mainloop.wait();
    }

    // Query sinks using introspection
    let sinks: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let done: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));

    let sinks_clone = Arc::clone(&sinks);
    let done_clone = Arc::clone(&done);

    let introspector = context.introspect();
    let _op = introspector.get_sink_info_list(move |result| {
        match result {
            pulse::callbacks::ListResult::Item(sink_info) => {
                let name = sink_info
                    .description
                    .as_ref()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| {
                        sink_info
                            .name
                            .as_ref()
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| "Unknown Output".to_string())
                    });
                if let Ok(mut list) = sinks_clone.lock() {
                    list.push(name);
                }
            }
            pulse::callbacks::ListResult::End | pulse::callbacks::ListResult::Error => {
                let (lock, cvar) = &*done_clone;
                if let Ok(mut finished) = lock.lock() {
                    *finished = true;
                    cvar.notify_one();
                }
            }
        }
    });

    // Wait for the sink list query to finish (with timeout)
    mainloop.unlock();
    {
        let (lock, cvar) = &*done;
        let mut finished = lock.lock().map_err(|e| e.to_string())?;
        let timeout = Duration::from_secs(3);
        while !*finished {
            let result = cvar.wait_timeout(finished, timeout).map_err(|e| e.to_string())?;
            finished = result.0;
            if result.1.timed_out() {
                break;
            }
        }
    }

    mainloop.stop();

    let devices = sinks.lock().map_err(|e| e.to_string())?.clone();

    if devices.is_empty() {
        Ok(vec!["Default Output".to_string()])
    } else {
        Ok(devices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_flat() {
        let mut filter = BiquadFilter::flat();

        // Verify coefficients for unity gain
        assert_eq!(filter.b0, 1.0);
        assert_eq!(filter.b1, 0.0);
        assert_eq!(filter.b2, 0.0);
        assert_eq!(filter.a1, 0.0);
        assert_eq!(filter.a2, 0.0);

        // Verify that it passes audio through unchanged
        let test_samples = [0.0, 0.5, -0.5, 1.0, -1.0];
        for &sample in &test_samples {
            assert_eq!(filter.process(sample, 0), sample);
            assert_eq!(filter.process(sample, 1), sample);
        }
    }
}
