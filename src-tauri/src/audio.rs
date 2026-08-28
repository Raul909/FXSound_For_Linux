//! Audio processing engine for FXSound.
//!
//! Provides a 10-band equalizer using biquad peak filters, audio effects
//! (fidelity, dynamic compression, bass boost, ambiance reverb, and 3D
//! surround widening), and real-time FFT-based spectrum analysis for the
//! visualizer.

use libpulse_binding as pulse;
use libpulse_simple_binding as psimple;
use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::collections::HashMap;
use std::sync::Arc;

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: u8 = 2;
const FFT_SIZE: usize = 512;

/// Center frequencies for the 10 EQ bands (Hz).
const EQ_FREQUENCIES: [f32; 10] = [
    32.0, 64.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

/// Corner frequency of the HyperBass low-shelf (Hz).
const BASS_SHELF_FREQ: f32 = 110.0;
/// Maximum low-shelf boost at HyperBass = 100 (dB). Deliberately moderate:
/// the bass-heavy EQ presets already add up to +10 dB down low, and the two
/// stack.
const BASS_SHELF_MAX_DB: f32 = 6.0;
/// Crossover of the one-pole high-band split feeding the Fidelity exciter (Hz).
const FIDELITY_CROSSOVER: f32 = 3000.0;

/// Smoothing coefficient for a one-pole envelope with the given time constant.
#[inline]
fn time_coef(seconds: f32, sample_rate: f32) -> f32 {
    1.0 - (-1.0 / (seconds * sample_rate)).exp()
}

/// Tiny constant mixed into recursive feedback paths so decaying tails settle
/// to zero rather than into denormal floats, which trap to microcode on x86
/// and can cost 10–100x per operation — enough to cause audible dropouts.
const ANTI_DENORMAL: f32 = 1e-20;

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

    /// Create a low-shelf filter — boosts/cuts everything below `freq` while
    /// leaving the midrange and treble alone.
    ///
    /// Used by HyperBass, which previously applied a flat broadband gain (i.e.
    /// a volume knob) rather than actually boosting the low end.
    ///
    /// - `freq` — shelf corner frequency in Hz
    /// - `gain_db` — boost/cut in dB applied below the corner
    /// - `slope` — shelf slope, 1.0 is the steepest without overshoot
    /// - `sample_rate` — audio sample rate in Hz
    fn low_shelf(freq: f32, gain_db: f32, slope: f32, sample_rate: f32) -> Self {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let cos_w0 = w0.cos();
        // RBJ cookbook: 2*sqrt(A)*alpha, expanded to avoid a separate alpha term
        let two_sqrt_a_alpha = w0.sin() * ((a * a + 1.0) * (1.0 / slope - 1.0) + 2.0 * a).sqrt();

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + two_sqrt_a_alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - two_sqrt_a_alpha);
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + two_sqrt_a_alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - two_sqrt_a_alpha;

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
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Process a single sample through the filter for the given channel.
    #[inline]
    fn process(&mut self, input: f32, channel: usize) -> f32 {
        let ch = channel % 2;
        let output = self.b0 * input + self.b1 * self.x1[ch] + self.b2 * self.x2[ch]
            - self.a1 * self.y1[ch]
            - self.a2 * self.y2[ch];

        // Shift delay line. The recursive y-terms decay towards denormals once
        // the input goes quiet, so flush them to zero.
        self.x2[ch] = self.x1[ch];
        self.x1[ch] = input;
        self.y2[ch] = self.y1[ch];
        self.y1[ch] = if output.abs() < 1e-25 { 0.0 } else { output };

        output
    }
}

// ──────────────────────────────────────────────
//  Reverb (Ambiance effect)
// ──────────────────────────────────────────────

/// One-pole-damped feedback comb filter (Freeverb-style).
struct CombFilter {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
    damp1: f32,
    damp2: f32,
    filter_store: f32,
}

impl CombFilter {
    fn new(size: usize, feedback: f32, damp: f32) -> Self {
        Self {
            buffer: vec![0.0; size.max(1)],
            index: 0,
            feedback,
            damp1: damp,
            damp2: 1.0 - damp,
            filter_store: 0.0,
        }
    }

    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];
        // Low-pass the feedback path for a warmer, less metallic tail
        self.filter_store = output * self.damp2 + self.filter_store * self.damp1 + ANTI_DENORMAL;
        self.buffer[self.index] = input + self.filter_store * self.feedback;
        self.index += 1;
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
        output
    }
}

/// Schroeder allpass filter used to diffuse the reverb tail.
struct AllpassFilter {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
}

impl AllpassFilter {
    fn new(size: usize, feedback: f32) -> Self {
        Self {
            buffer: vec![0.0; size.max(1)],
            index: 0,
            feedback,
        }
    }

    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let buffered = self.buffer[self.index];
        let output = -input + buffered;
        self.buffer[self.index] = input + buffered * self.feedback;
        self.index += 1;
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
        output
    }
}

/// Compact stereo reverb (4 parallel combs + 2 series allpasses per channel),
/// a reduced Freeverb. Produces the wet signal for the "ambiance" effect.
///
/// Delay lengths are tuned for a 48 kHz sample rate; the right channel is
/// offset by a small stereo spread so the two channels decorrelate.
struct StereoReverb {
    combs_l: Vec<CombFilter>,
    combs_r: Vec<CombFilter>,
    allpass_l: Vec<AllpassFilter>,
    allpass_r: Vec<AllpassFilter>,
    input_gain: f32,
}

impl StereoReverb {
    fn new() -> Self {
        // Comb/allpass delay lengths in samples (Freeverb tunings scaled to 48 kHz)
        const COMB_TUNINGS: [usize; 4] = [1215, 1293, 1390, 1476];
        const ALLPASS_TUNINGS: [usize; 2] = [605, 480];
        const STEREO_SPREAD: usize = 25;
        const ROOM_SIZE: f32 = 0.82; // comb feedback — larger = longer tail
        const DAMP: f32 = 0.25; // high-frequency damping of the tail
        const ALLPASS_FEEDBACK: f32 = 0.5;

        let combs_l = COMB_TUNINGS
            .iter()
            .map(|&t| CombFilter::new(t, ROOM_SIZE, DAMP))
            .collect();
        let combs_r = COMB_TUNINGS
            .iter()
            .map(|&t| CombFilter::new(t + STEREO_SPREAD, ROOM_SIZE, DAMP))
            .collect();
        let allpass_l = ALLPASS_TUNINGS
            .iter()
            .map(|&t| AllpassFilter::new(t, ALLPASS_FEEDBACK))
            .collect();
        let allpass_r = ALLPASS_TUNINGS
            .iter()
            .map(|&t| AllpassFilter::new(t + STEREO_SPREAD, ALLPASS_FEEDBACK))
            .collect();

        Self {
            combs_l,
            combs_r,
            allpass_l,
            allpass_r,
            // Scales the dry input feeding the reverb so the summed comb
            // output stays near unity before the wet mix is applied.
            input_gain: 0.022,
        }
    }

    /// Process one stereo frame and return the wet (reverb-only) L/R signal.
    #[inline]
    fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        let input = (l + r) * self.input_gain;

        // Parallel comb filters (summed)
        let mut wet_l = 0.0;
        for comb in self.combs_l.iter_mut() {
            wet_l += comb.process(input);
        }
        let mut wet_r = 0.0;
        for comb in self.combs_r.iter_mut() {
            wet_r += comb.process(input);
        }

        // Series allpass filters (diffusion)
        for ap in self.allpass_l.iter_mut() {
            wet_l = ap.process(wet_l);
        }
        for ap in self.allpass_r.iter_mut() {
            wet_r = ap.process(wet_r);
        }

        (wet_l, wet_r)
    }
}

// ──────────────────────────────────────────────
//  Dynamics
// ──────────────────────────────────────────────

/// Stereo-linked peak limiter with a smoothed gain envelope.
///
/// Replaces the previous per-buffer normalisation, which computed a single
/// scale factor for each 1024-sample block and applied it uniformly. That
/// stepped the gain at every block boundary, so any material driven past
/// 0 dBFS picked up ~90 Hz amplitude modulation (audible pumping and zipper
/// noise) instead of transparent limiting. Here the gain moves sample by
/// sample and persists across buffers, so block boundaries are inaudible.
struct Limiter {
    gain: f32,
    attack: f32,
    release: f32,
    threshold: f32,
}

impl Limiter {
    fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            attack: time_coef(0.5e-3, sample_rate),
            release: time_coef(120e-3, sample_rate),
            threshold: 0.98,
        }
    }

    #[inline]
    fn process_frame(&mut self, l: &mut f32, r: &mut f32) {
        let peak = l.abs().max(r.abs());
        let target = if peak > self.threshold {
            self.threshold / peak
        } else {
            1.0
        };

        // Pull the gain down quickly, let it recover slowly.
        let coef = if target < self.gain {
            self.attack
        } else {
            self.release
        };
        self.gain += (target - self.gain) * coef;

        *l *= self.gain;
        *r *= self.gain;

        // Safety net: with no lookahead the envelope still lags the very first
        // sample of a fast transient, so clamp rather than let it clip the sink.
        *l = l.clamp(-1.0, 1.0);
        *r = r.clamp(-1.0, 1.0);
    }
}

/// Stereo-linked compressor with makeup gain, driving the "Dynamic Boost" effect.
///
/// The previous implementation was an instantaneous hard-knee waveshaper: it
/// folded every sample above the threshold with no envelope and no makeup gain,
/// so raising the slider made the output quieter and added harmonic distortion
/// — the opposite of the "boost" on the label. This version follows the
/// envelope with proper attack/release and restores the headroom it takes.
struct Compressor {
    env: f32,
    attack: f32,
    release: f32,
}

impl Compressor {
    fn new(sample_rate: f32) -> Self {
        Self {
            env: 0.0,
            attack: time_coef(5e-3, sample_rate),
            release: time_coef(80e-3, sample_rate),
        }
    }

    /// - `threshold` — linear level above which gain reduction starts
    /// - `slope` — 1/ratio (1.0 = no compression, 0.4 = 2.5:1)
    /// - `makeup` — output gain applied after compression
    #[inline]
    fn process_frame(&mut self, l: &mut f32, r: &mut f32, threshold: f32, slope: f32, makeup: f32) {
        let peak = l.abs().max(r.abs());
        let coef = if peak > self.env {
            self.attack
        } else {
            self.release
        };
        self.env += (peak - self.env) * coef;

        let reduction = if self.env > threshold {
            (threshold + (self.env - threshold) * slope) / self.env
        } else {
            1.0
        };

        let g = reduction * makeup;
        *l *= g;
        *r *= g;
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
    /// Cached FFT processor and buffer to avoid repeated allocations.
    fft_processor: Arc<dyn Fft<f32>>,
    complex_buffer: Vec<Complex<f32>>,

    powered: bool,
    eq_bands: [f32; 10],
    effects: HashMap<String, f32>,
    sample_rate: u32,

    /// One biquad filter per EQ band — rebuilt when gain changes.
    filters: Vec<BiquadFilter>,

    /// FFT magnitude data shared with the UI for the visualizer.
    pub fft_data: Arc<std::sync::Mutex<Vec<f32>>>,

    /// Precomputed FFT bin boundaries for mapping to 32 visualizer bars.
    fft_bin_boundaries: [usize; 33],

    /// Stereo reverb driving the "ambiance" effect (spatial ambience).
    reverb: StereoReverb,

    /// Low-shelf filter implementing HyperBass — rebuilt when the slider moves.
    bass_shelf: BiquadFilter,

    /// One-pole low-pass state (per channel) used to split off the high band
    /// that the Fidelity exciter saturates.
    fidelity_lp: [f32; 2],
    fidelity_lp_coef: f32,

    /// Dynamics stages, held across buffers so their envelopes stay continuous.
    compressor: Compressor,
    limiter: Limiter,

    /// Hann window applied before the visualizer FFT to suppress the spectral
    /// leakage that made neighbouring bars bleed into each other.
    fft_window: Vec<f32>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let mut planner = FftPlanner::new();
        let fft_processor = planner.plan_fft_forward(FFT_SIZE);
        let complex_buffer = vec![Complex::new(0.0, 0.0); FFT_SIZE];

        // Start with flat (0 dB) filters for all 10 bands
        let filters = EQ_FREQUENCIES
            .iter()
            .map(|_| BiquadFilter::flat())
            .collect();

        let mut fft_bin_boundaries = [0usize; 33];
        fft_bin_boundaries[0] = 1;
        let mut bin_low = 1;
        for i in 0..32 {
            let next_index = 1.0 + ((i + 1) as f32 / 32.0).powf(1.8) * 255.0;
            let mut bin_high = next_index.round() as usize;
            if bin_high <= bin_low {
                bin_high = bin_low + 1;
            }
            bin_high = bin_high.min(FFT_SIZE / 2);
            fft_bin_boundaries[i + 1] = bin_high;
            bin_low = bin_high;
        }

        // Periodic Hann window, matching the FFT's implicit periodicity.
        let fft_window = (0..FFT_SIZE)
            .map(|n| {
                0.5 * (1.0
                    - (2.0 * std::f32::consts::PI * n as f32 / FFT_SIZE as f32).cos())
            })
            .collect();

        let sample_rate = SAMPLE_RATE as f32;

        Self {
            fft_processor,
            complex_buffer,
            powered: true,
            eq_bands: [0.0; 10],
            effects: HashMap::new(),
            sample_rate: SAMPLE_RATE,
            filters,
            fft_data: Arc::new(std::sync::Mutex::new(vec![0.0; 32])),
            fft_bin_boundaries,
            reverb: StereoReverb::new(),
            bass_shelf: BiquadFilter::flat(),
            fidelity_lp: [0.0; 2],
            fidelity_lp_coef: 1.0
                - (-2.0 * std::f32::consts::PI * FIDELITY_CROSSOVER / sample_rate).exp(),
            compressor: Compressor::new(sample_rate),
            limiter: Limiter::new(sample_rate),
            fft_window,
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

        // HyperBass runs through a low-shelf biquad, so its coefficients have
        // to be recomputed whenever the slider moves.
        if effect == "bass" {
            self.bass_shelf = if clamped < 0.5 {
                BiquadFilter::flat()
            } else {
                BiquadFilter::low_shelf(
                    BASS_SHELF_FREQ,
                    (clamped / 100.0) * BASS_SHELF_MAX_DB,
                    0.9,
                    self.sample_rate as f32,
                )
            };
        }

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
        self.fft_data
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    // ── Main processing pipeline ──

    /// Process an audio buffer: apply EQ, effects, limiter, and update the visualizer.
    ///
    /// When powered off, the output is filled with silence to prevent
    /// audio doubling (the original system audio is already playing).
    pub fn process_audio(&mut self, input: &[f32], output: &mut [f32]) {
        if !self.powered {
            output.fill(0.0);
            // Let the visualizer fall to the floor instead of freezing on the
            // last frame captured before power-off.
            self.decay_fft();
            return;
        }

        // Skip near-silent input to avoid amplifying noise
        // Optimization: short-circuit the sum of squares calculation as soon as the threshold is met
        let threshold = 0.000001 * input.len() as f32;
        let mut sum_sq = 0.0;
        for &x in input.iter() {
            sum_sq += x * x;
            if sum_sq >= threshold {
                break;
            }
        }
        if sum_sq < threshold {
            output.fill(0.0);
            self.decay_fft();
            return;
        }

        // Apply the 10-band EQ using biquad filters
        self.apply_eq(input, output);

        // Apply audio effects (fidelity, dynamic, bass, 3D surround, ambiance)
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

        // Pre-compute active bands to avoid branching in the inner loop
        let mut active_bands = [0usize; 10];
        let mut active_count = 0;
        for band in 0..10 {
            // Skip flat bands for efficiency
            if self.eq_bands[band].abs() >= 0.1 {
                active_bands[active_count] = band;
                active_count += 1;
            }
        }

        let active_bands_slice = &active_bands[..active_count];

        // Process each sample through all active biquad filters
        // Interleaved stereo: chunk[0] = left, chunk[1] = right
        for chunk in output.chunks_exact_mut(CHANNELS as usize) {
            let mut l = chunk[0];
            let mut r = chunk[1];

            for &band in active_bands_slice {
                // Process left channel
                l = self.filters[band].process(l, 0);
                // Process right channel
                r = self.filters[band].process(r, 1);
            }

            chunk[0] = l;
            chunk[1] = r;
        }
    }

    // ── Effects Processing ──

    /// Apply audio effects to the buffer.
    ///
    /// Chain order: HyperBass (low shelf) → Fidelity (high-band exciter) →
    /// 3D surround (mid/side stereo widening) → ambiance (stereo reverb mixed
    /// in as a wet send) → Dynamic Boost (compressor with makeup gain).
    ///
    /// Dynamics run last so the compressor sees the finished signal — putting
    /// it mid-chain, as before, meant later stages could re-introduce the peaks
    /// it had just controlled.
    fn apply_effects(&mut self, buffer: &mut [f32]) {
        let fidelity = self.effects.get("fidelity").copied().unwrap_or(0.0);
        let dynamic = self.effects.get("dynamic").copied().unwrap_or(0.0);
        let bass = self.effects.get("bass").copied().unwrap_or(0.0);
        let ambiance = self.effects.get("ambiance").copied().unwrap_or(0.0);
        let surround = self.effects.get("surround").copied().unwrap_or(0.0);

        // ── HyperBass: low-shelf boost below ~110 Hz ──
        // Previously a flat broadband multiply, which just made everything
        // louder without changing the tonal balance at all.
        if bass >= 0.5 {
            for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
                frame[0] = self.bass_shelf.process(frame[0], 0);
                frame[1] = self.bass_shelf.process(frame[1], 1);
            }
        }

        // ── Fidelity: high-band harmonic exciter ──
        // The old version saturated the full-band signal, which mostly added
        // intermodulation on the bass. Split off the band above ~3 kHz, drive
        // only that, and add it back on top of the untouched dry signal.
        if fidelity > 0.0 {
            let amount = fidelity / 100.0;
            let drive = 1.5 + amount * 2.5;
            let mix = amount * 0.30;
            let coef = self.fidelity_lp_coef;

            for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
                for (sample, lp) in frame.iter_mut().zip(self.fidelity_lp.iter_mut()) {
                    let s = *sample;
                    *lp += (s - *lp) * coef;
                    let high = s - *lp;
                    *sample = s + (high * drive).tanh() * mix;
                }
            }
        }

        // ── 3D Surround: mid/side stereo widening ──
        // width scales from 1.0 (no change) at 0 to 2.0 at 100. The mid
        // (mono) component is preserved, so mono content and downmix
        // compatibility are unaffected — only the stereo "side" is widened.
        if surround > 0.0 {
            let width = 1.0 + (surround / 100.0);
            for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
                let mid = (frame[0] + frame[1]) * 0.5;
                let side = (frame[0] - frame[1]) * 0.5 * width;
                frame[0] = mid + side;
                frame[1] = mid - side;
            }
        }

        // ── Ambiance: stereo reverb mixed on top of the dry signal ──
        // Runs as a parallel "send": the dry signal is kept intact and a
        // scaled wet reverb is added, so raising ambiance adds space without
        // hollowing out the original. The limiter downstream tames peaks.
        if ambiance > 0.0 {
            let wet = (ambiance / 100.0) * 0.45;
            for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
                let (wet_l, wet_r) = self.reverb.process(frame[0], frame[1]);
                frame[0] += wet_l * wet;
                frame[1] += wet_r * wet;
            }
        }

        // ── Dynamic Boost: compression with makeup gain ──
        // Narrows the gap between quiet and loud passages, then gives back the
        // headroom that compression removed so the result is audibly louder and
        // denser — which is what the slider name promises.
        if dynamic > 0.0 {
            let amount = dynamic / 100.0;
            let threshold = 1.0 - 0.45 * amount;
            let slope = 1.0 - 0.6 * amount;
            // Gain that restores a full-scale peak back to full scale.
            let makeup = 1.0 / (threshold + (1.0 - threshold) * slope);

            for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
                let (mut l, mut r) = (frame[0], frame[1]);
                self.compressor
                    .process_frame(&mut l, &mut r, threshold, slope, makeup);
                frame[0] = l;
                frame[1] = r;
            }
        }
    }

    // ── Limiter ──

    /// Catch peaks above the ceiling with a smoothed, sample-accurate gain
    /// envelope (see [`Limiter`]).
    fn apply_limiter(&mut self, buffer: &mut [f32]) {
        for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
            let (mut l, mut r) = (frame[0], frame[1]);
            self.limiter.process_frame(&mut l, &mut r);
            frame[0] = l;
            frame[1] = r;
        }
    }

    // ── Visualizer FFT ──

    /// Fade the visualizer bars towards zero.
    ///
    /// Called on the silent and powered-off paths, which return before the FFT
    /// runs. Without this the last computed magnitudes stayed latched and the
    /// bars froze mid-height whenever playback stopped.
    fn decay_fft(&mut self) {
        let mut fft_data = self.fft_data.lock().unwrap_or_else(|e| e.into_inner());
        for value in fft_data.iter_mut() {
            *value *= 0.75;
            if *value < 0.01 {
                *value = 0.0;
            }
        }
    }

    /// Compute FFT magnitudes from the output buffer and store for the visualizer.
    fn update_fft(&mut self, buffer: &[f32]) {
        // Since input is stereo (interleaved), we need at least FFT_SIZE * 2 samples
        if buffer.len() < FFT_SIZE * 2 {
            return;
        }

        // Mix interleaved stereo to mono into the complex buffer, applying the
        // Hann window. Without a window the abrupt block edges smear energy
        // across every bin, so a pure tone lit up bars either side of it.
        for ((chunk, complex), w) in buffer
            .chunks_exact(2)
            .zip(self.complex_buffer.iter_mut())
            .zip(self.fft_window.iter())
        {
            let mono = (chunk[0] + chunk[1]) * 0.5 * w;
            *complex = Complex::new(mono, 0.0);
        }

        self.fft_processor.process(&mut self.complex_buffer);

        // Convert to magnitudes and map to 32 bands exponentially (log-like spacing)
        let mut fft_data = self.fft_data.lock().unwrap_or_else(|e| e.into_inner());

        for i in 0..32 {
            let bin_low = self.fft_bin_boundaries[i];
            let bin_high = self.fft_bin_boundaries[i + 1];

            let mut max_val = 0.0f32;
            let mut sum_val = 0.0f32;
            let count = bin_high - bin_low;

            for bin in bin_low..bin_high {
                let mag = self.complex_buffer[bin].norm();
                max_val = max_val.max(mag);
                sum_val += mag;
            }

            let avg_val = sum_val / count as f32;
            // Blend peak and average for visually appealing and responsive bars.
            // The 300 factor is the previous 150 divided by the Hann window's
            // coherent gain of 0.5, so bar heights match the pre-window build.
            let val = (avg_val * 0.3 + max_val * 0.7) * 300.0;

            fft_data[i] = val.min(100.0);
        }
    }
}

// ──────────────────────────────────────────────
//  PulseAudio Integration
// ──────────────────────────────────────────────

/// Shared handle for retargeting the playback stream while the audio loop runs.
///
/// The loop polls `generation` once per buffer; bumping it makes the loop tear
/// down its playback stream and reopen it on the newly requested sink. Without
/// this the "Output Device" dropdown was inert — the loop always opened the
/// server default and nothing the user picked had any effect.
#[derive(Clone, Default)]
pub struct OutputRouting {
    sink: Arc<std::sync::Mutex<Option<String>>>,
    generation: Arc<std::sync::atomic::AtomicU64>,
}

impl OutputRouting {
    /// Request playback on a specific sink, or `None` for the server default.
    pub fn set_sink(&self, name: Option<String>) {
        *self.sink.lock().unwrap_or_else(|e| e.into_inner()) = name;
        self.generation
            .fetch_add(1, std::sync::atomic::Ordering::Release);
    }

    fn sink(&self) -> Option<String> {
        self.sink.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    fn generation(&self) -> u64 {
        self.generation.load(std::sync::atomic::Ordering::Acquire)
    }
}

/// Manages the PulseAudio capture/playback loop.
///
/// Captures system audio from the monitor source, runs it through
/// the AudioEngine for processing, and outputs the result.
pub struct AudioProcessor {
    engine: Arc<std::sync::Mutex<AudioEngine>>,
    routing: OutputRouting,
}

impl AudioProcessor {
    pub fn new(engine: Arc<std::sync::Mutex<AudioEngine>>, routing: OutputRouting) -> Self {
        Self { engine, routing }
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
        let routing = self.routing.clone();

        std::thread::spawn(move || match Self::audio_loop(engine, routing, spec) {
            Ok(_) => log::info!("Audio loop ended normally"),
            Err(e) => log::error!("Audio loop error: {}", e),
        });

        Ok(())
    }

    /// Open a playback stream on `sink` (or the server default when `None`).
    fn open_output(
        spec: &pulse::sample::Spec,
        sink: Option<&str>,
    ) -> Result<psimple::Simple, String> {
        psimple::Simple::new(
            None,
            "FXSound Output",
            pulse::stream::Direction::Playback,
            sink,
            "Processed Audio",
            spec,
            None,
            None,
        )
        .map_err(|e| format!("Failed to create output stream: {}", e))
    }

    /// Main audio capture → process → playback loop.
    ///
    /// Reads from the system monitor source (captures all desktop audio),
    /// processes it through the AudioEngine, and writes to an output stream.
    fn audio_loop(
        engine: Arc<std::sync::Mutex<AudioEngine>>,
        routing: OutputRouting,
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
        )
        .inspect_err(|e| {
            log::warn!(
                "Failed to open monitor source: {}. Trying default source...",
                e
            );
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
                )
                .map_err(|e| format!("Failed to create input stream: {}", e))?
            }
        };

        // Create the playback output stream on whichever sink is selected.
        let mut output_generation = routing.generation();
        let mut output = Self::open_output(&spec, routing.sink().as_deref())?;

        log::info!("Audio streams created successfully");
        log::info!("Processing system audio through FXSound...");

        const BUFFER_SIZE: usize = 1024;
        let mut input_bytes = vec![0u8; BUFFER_SIZE * 4]; // f32 = 4 bytes
        let mut input_samples = vec![0f32; BUFFER_SIZE];
        let mut output_samples = vec![0f32; BUFFER_SIZE];
        let mut output_bytes = vec![0u8; BUFFER_SIZE * 4];

        loop {
            // Reopen the playback stream if the user picked a different output
            // device. On failure keep the working stream rather than going
            // silent — a bad choice should not kill audio.
            let generation = routing.generation();
            if generation != output_generation {
                output_generation = generation;
                let requested = routing.sink();
                match Self::open_output(&spec, requested.as_deref()) {
                    Ok(stream) => {
                        output = stream;
                        log::info!(
                            "Output device switched to {}",
                            requested.as_deref().unwrap_or("system default")
                        );
                    }
                    Err(e) => log::error!("Keeping previous output device: {}", e),
                }
            }

            // Read raw bytes from the input stream
            if let Err(e) = input.read(&mut input_bytes) {
                log::error!("Read error: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            // Convert bytes to f32 samples in-place
            for (chunk, sample) in input_bytes.chunks_exact(4).zip(input_samples.iter_mut()) {
                *sample = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            }

            // Process audio through the engine
            {
                let mut engine = engine.lock().unwrap_or_else(|e| e.into_inner());
                engine.process_audio(&input_samples, &mut output_samples);
            }

            // Convert f32 samples back to bytes in-place
            for (sample, chunk) in output_samples.iter().zip(output_bytes.chunks_exact_mut(4)) {
                chunk.copy_from_slice(&sample.to_le_bytes());
            }

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

/// A PulseAudio/PipeWire playback sink.
///
/// `name` is the stable identifier the playback stream is opened against;
/// `description` is what the user sees. The two are different strings, which is
/// why listing descriptions alone was not enough to actually route audio.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AudioSink {
    pub name: String,
    pub description: String,
    pub is_default: bool,
}

/// Query PulseAudio for available audio output sinks.
///
/// Uses the introspection API to list every sink along with the server's
/// current default, so the UI can preselect the device audio is really on.
pub fn get_pulse_sinks() -> Result<Vec<AudioSink>, String> {
    use pulse::context::{Context, FlagSet as ContextFlagSet};
    use pulse::mainloop::threaded::Mainloop;
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::Duration;

    /// Block until a callback signals completion, or time out.
    fn wait_for(done: &Arc<(Mutex<bool>, Condvar)>) {
        let (lock, cvar) = &**done;
        let mut finished = lock.lock().unwrap_or_else(|e| e.into_inner());
        let timeout = Duration::from_secs(3);
        while !*finished {
            let (guard, result) = cvar
                .wait_timeout(finished, timeout)
                .unwrap_or_else(|e| e.into_inner());
            finished = guard;
            if result.timed_out() {
                break;
            }
        }
    }

    fn signal(done: &Arc<(Mutex<bool>, Condvar)>) {
        let (lock, cvar) = &**done;
        let mut finished = lock.lock().unwrap_or_else(|e| e.into_inner());
        *finished = true;
        cvar.notify_one();
    }

    // Create a threaded mainloop for the introspection query
    let mut mainloop = Mainloop::new().ok_or("Failed to create PulseAudio mainloop")?;
    mainloop
        .start()
        .map_err(|e| format!("Failed to start mainloop: {}", e))?;

    let mut context = match Context::new(&mainloop, "FXSound Device Query") {
        Some(context) => context,
        None => {
            mainloop.stop();
            return Err("Failed to create PulseAudio context".to_string());
        }
    };

    // Lock the mainloop while connecting
    mainloop.lock();
    if let Err(e) = context.connect(None, ContextFlagSet::NOFLAGS, None) {
        mainloop.unlock();
        mainloop.stop();
        return Err(format!("Failed to connect context: {}", e));
    }

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

    let introspector = context.introspect();

    // ── Which sink is the server default? ──
    let default_sink: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let server_done: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let sink_slot = Arc::clone(&default_sink);
        let done = Arc::clone(&server_done);
        let _op = introspector.get_server_info(move |info| {
            if let Some(name) = info.default_sink_name.as_ref() {
                *sink_slot.lock().unwrap_or_else(|e| e.into_inner()) = Some(name.to_string());
            }
            signal(&done);
        });
        mainloop.unlock();
        wait_for(&server_done);
        mainloop.lock();
    }
    let default_sink = default_sink.lock().unwrap_or_else(|e| e.into_inner()).clone();

    // ── Enumerate the sinks ──
    let sinks: Arc<Mutex<Vec<AudioSink>>> = Arc::new(Mutex::new(Vec::new()));
    let list_done: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let collected = Arc::clone(&sinks);
        let done = Arc::clone(&list_done);
        let default_sink = default_sink.clone();
        let _op = introspector.get_sink_info_list(move |result| match result {
            pulse::callbacks::ListResult::Item(sink_info) => {
                let name = match sink_info.name.as_ref() {
                    Some(name) => name.to_string(),
                    // Without a name we cannot open a stream against it.
                    None => return,
                };
                let description = sink_info
                    .description
                    .as_ref()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| name.clone());
                let is_default = default_sink.as_deref() == Some(name.as_str());
                if let Ok(mut list) = collected.lock() {
                    list.push(AudioSink {
                        name,
                        description,
                        is_default,
                    });
                }
            }
            pulse::callbacks::ListResult::End | pulse::callbacks::ListResult::Error => {
                signal(&done);
            }
        });
        mainloop.unlock();
        wait_for(&list_done);
    }

    mainloop.stop();

    let mut devices = sinks.lock().unwrap_or_else(|e| e.into_inner()).clone();

    // Show the active device first so the dropdown opens on the right one.
    devices.sort_by_key(|sink| !sink.is_default);

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RMS of a buffer.
    fn rms(b: &[f32]) -> f32 {
        (b.iter().map(|x| x * x).sum::<f32>() / b.len() as f32).sqrt()
    }

    /// Interleaved stereo sine at `freq` Hz, `n_frames` frames long.
    fn stereo_sine(freq: f32, amplitude: f32, n_frames: usize) -> Vec<f32> {
        let mut buf = vec![0.0f32; n_frames * 2];
        for (i, frame) in buf.chunks_exact_mut(2).enumerate() {
            let s = amplitude
                * (2.0 * std::f32::consts::PI * freq * i as f32 / SAMPLE_RATE as f32).sin();
            frame[0] = s;
            frame[1] = s;
        }
        buf
    }

    /// Run `input` through the engine repeatedly so filter/envelope state
    /// settles, and return the final output buffer.
    fn settle(engine: &mut AudioEngine, input: &[f32], passes: usize) -> Vec<f32> {
        let mut output = vec![0.0f32; input.len()];
        for _ in 0..passes {
            engine.process_audio(input, &mut output);
        }
        output
    }

    /// Gain in dB that the engine applied to `input`.
    fn gain_db(input: &[f32], output: &[f32]) -> f32 {
        20.0 * (rms(output) / rms(input)).log10()
    }

    /// Mirror of the preset tables shipped in `src/constants.js`. Kept here so
    /// the DSP can be checked against the values users actually load; if the
    /// frontend presets change, update these to match.
    const PRESET_NAMES: [&str; 10] = [
        "Flat",
        "Music",
        "Movies",
        "Gaming",
        "Podcast",
        "Bass Boost",
        "Vocal Boost",
        "Deep Bass",
        "Treble Boost",
        "Night Mode",
    ];
    const PRESET_EQ: [[f32; 10]; 10] = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [3.0, 2.0, 1.0, 0.0, -1.0, 0.0, 2.0, 3.0, 3.0, 2.0],
        [4.0, 3.0, 2.0, 0.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0],
        [5.0, 4.0, 2.0, 1.0, 0.0, 0.0, 1.0, 2.0, 4.0, 5.0],
        [-1.0, 0.0, 2.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0],
        [8.0, 7.0, 5.0, 3.0, 0.0, -1.0, -1.0, -1.0, -2.0, -2.0],
        [-2.0, -1.0, 0.0, 3.0, 5.0, 5.0, 3.0, 1.0, 0.0, -1.0],
        [10.0, 8.0, 6.0, 2.0, 0.0, -1.0, -2.0, -2.0, -3.0, -3.0],
        [-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 4.0, 6.0, 7.0, 8.0],
        [2.0, 2.0, 1.0, 0.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0],
    ];
    /// fidelity, ambiance, dynamic, surround, bass
    const PRESET_FX: [[f32; 5]; 10] = [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [65.0, 40.0, 50.0, 30.0, 45.0],
        [70.0, 75.0, 60.0, 80.0, 55.0],
        [60.0, 50.0, 70.0, 90.0, 60.0],
        [80.0, 20.0, 55.0, 10.0, 20.0],
        [50.0, 30.0, 65.0, 20.0, 90.0],
        [85.0, 35.0, 50.0, 25.0, 15.0],
        [45.0, 25.0, 70.0, 15.0, 95.0],
        [75.0, 45.0, 45.0, 35.0, 10.0],
        [55.0, 60.0, 35.0, 40.0, 30.0],
    ];

    fn load_preset(engine: &mut AudioEngine, index: usize) {
        for (band, &gain) in PRESET_EQ[index].iter().enumerate() {
            engine.set_eq_band(band, gain);
        }
        let fx = PRESET_FX[index];
        engine.set_effect("fidelity", fx[0]);
        engine.set_effect("ambiance", fx[1]);
        engine.set_effect("dynamic", fx[2]);
        engine.set_effect("surround", fx[3]);
        engine.set_effect("bass", fx[4]);
    }

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

    #[test]
    fn test_pipeline_identity_at_defaults() {
        // With flat EQ and all effects at zero, processing must be a
        // pass-through for a non-silent, in-range stereo signal.
        let mut engine = AudioEngine::new();
        let input: Vec<f32> = (0..1024).map(|i| 0.2 * (i as f32 * 0.05).sin()).collect();
        let mut output = vec![0.0f32; input.len()];
        engine.process_audio(&input, &mut output);
        for (a, b) in input.iter().zip(output.iter()) {
            assert!((a - b).abs() < 1e-4, "pipeline not identity at defaults");
        }
    }

    #[test]
    fn test_surround_preserves_mono() {
        // Mid/side widening must leave mono content (L == R) untouched at any
        // width, because the widened "side" component is zero.
        let mut engine = AudioEngine::new();
        engine.set_effect("surround", 100.0);
        let input: Vec<f32> = vec![0.3; 1024]; // L == R everywhere
        let mut output = vec![0.0f32; input.len()];
        engine.process_audio(&input, &mut output);
        for (a, b) in input.iter().zip(output.iter()) {
            assert!((a - b).abs() < 1e-4, "surround altered mono content");
        }
    }

    #[test]
    fn test_ambiance_is_finite_bounded_and_active() {
        // The reverb must stay finite, respect the limiter ceiling, and
        // audibly change the signal once its tail has built up.
        let mut engine = AudioEngine::new();
        engine.set_effect("ambiance", 100.0);
        let input: Vec<f32> = (0..2048).map(|i| 0.3 * (i as f32 * 0.1).sin()).collect();
        let mut output = vec![0.0f32; input.len()];
        for _ in 0..4 {
            engine.process_audio(&input, &mut output);
        }
        let mut changed = false;
        for (a, b) in input.iter().zip(output.iter()) {
            assert!(b.is_finite(), "reverb produced non-finite output");
            assert!(b.abs() <= 1.0001, "reverb output exceeded limiter ceiling");
            if (a - b).abs() > 1e-3 {
                changed = true;
            }
        }
        assert!(changed, "ambiance did not alter the signal");
    }

    #[test]
    fn measure_default_music_preset_levels() {
        // The default "Music" preset now activates ambiance (40) and surround
        // (30). Confirm those defaults keep the overall level musically sane —
        // a clipping or washed-out default would be a bad upgrade experience.
        let mut engine = AudioEngine::new();
        engine.set_effect("ambiance", 40.0);
        engine.set_effect("surround", 30.0);

        // Broadband-ish stereo signal, slightly decorrelated so surround engages.
        let mut input = vec![0.0f32; 2048];
        for (i, frame) in input.chunks_exact_mut(2).enumerate() {
            let t = i as f32;
            frame[0] = 0.25 * (t * 0.11).sin() + 0.10 * (t * 0.37).sin();
            frame[1] = 0.25 * (t * 0.11 + 0.4).sin() + 0.10 * (t * 0.29).sin();
        }
        let mut output = vec![0.0f32; input.len()];
        for _ in 0..8 {
            engine.process_audio(&input, &mut output); // let the reverb tail settle
        }

        let rms = |b: &[f32]| (b.iter().map(|x| x * x).sum::<f32>() / b.len() as f32).sqrt();
        let in_rms = rms(&input);
        let out_rms = rms(&output);
        let gain_db = 20.0 * (out_rms / in_rms).log10();
        println!(
            "Music-preset defaults: in_rms={in_rms:.4} out_rms={out_rms:.4} gain={gain_db:+.2} dB"
        );

        // Not inaudible, not a wall of reverb/clipping.
        assert!(
            gain_db > -6.0 && gain_db < 6.0,
            "unexpected default level change: {gain_db:+.2} dB"
        );
    }

    #[test]
    fn test_hyperbass_boosts_lows_and_leaves_highs_alone() {
        // HyperBass used to be a flat broadband multiply — a volume knob, not a
        // bass control. It must now lift the low end and leave treble untouched.
        let low_in = stereo_sine(60.0, 0.2, 1024);
        let high_in = stereo_sine(8000.0, 0.2, 1024);

        let mut low_engine = AudioEngine::new();
        low_engine.set_effect("bass", 100.0);
        let low_gain = gain_db(&low_in, &settle(&mut low_engine, &low_in, 4));

        let mut high_engine = AudioEngine::new();
        high_engine.set_effect("bass", 100.0);
        let high_gain = gain_db(&high_in, &settle(&mut high_engine, &high_in, 4));

        println!("HyperBass @100: 60 Hz {low_gain:+.2} dB, 8 kHz {high_gain:+.2} dB");
        assert!(
            low_gain > 4.0,
            "60 Hz should be clearly boosted, got {low_gain:+.2} dB"
        );
        assert!(
            high_gain.abs() < 1.0,
            "8 kHz should be untouched, got {high_gain:+.2} dB"
        );
    }

    #[test]
    fn test_fidelity_excites_highs_not_lows() {
        // Fidelity is documented as high-frequency enhancement; the old
        // full-band saturator coloured the bass instead.
        let low_in = stereo_sine(80.0, 0.2, 1024);
        let high_in = stereo_sine(9000.0, 0.2, 1024);

        let mut low_engine = AudioEngine::new();
        low_engine.set_effect("fidelity", 100.0);
        let low_gain = gain_db(&low_in, &settle(&mut low_engine, &low_in, 4));

        let mut high_engine = AudioEngine::new();
        high_engine.set_effect("fidelity", 100.0);
        let high_gain = gain_db(&high_in, &settle(&mut high_engine, &high_in, 4));

        println!("Fidelity @100: 80 Hz {low_gain:+.2} dB, 9 kHz {high_gain:+.2} dB");
        assert!(
            low_gain.abs() < 0.5,
            "bass should pass through Fidelity untouched, got {low_gain:+.2} dB"
        );
        assert!(
            high_gain > 0.5,
            "highs should be lifted by Fidelity, got {high_gain:+.2} dB"
        );
    }

    #[test]
    fn test_dynamic_boost_makes_signal_louder() {
        // The previous implementation attenuated and distorted: it folded peaks
        // with no makeup gain, so "Dynamic Boost" turned the volume down.
        let input = stereo_sine(440.0, 0.5, 1024);
        let mut engine = AudioEngine::new();
        engine.set_effect("dynamic", 100.0);
        let out = settle(&mut engine, &input, 8);
        let g = gain_db(&input, &out);

        println!("Dynamic Boost @100: {g:+.2} dB");
        assert!(g > 0.5, "Dynamic Boost should raise level, got {g:+.2} dB");
        assert!(
            out.iter().all(|s| s.is_finite() && s.abs() <= 1.0001),
            "Dynamic Boost produced out-of-range output"
        );
    }

    #[test]
    fn test_limiter_gain_is_continuous_across_buffers() {
        // The old limiter normalised each 1024-sample block by its own peak, so
        // the gain stepped at every block boundary — ~90 Hz pumping on anything
        // driven past 0 dBFS. With a smoothed envelope the seam must vanish.
        let input = stereo_sine(220.0, 1.6, 512); // deliberately over full scale
        let mut engine = AudioEngine::new();

        let mut prev = vec![0.0f32; input.len()];
        engine.process_audio(&input, &mut prev);
        let mut curr = vec![0.0f32; input.len()];
        for _ in 0..6 {
            std::mem::swap(&mut prev, &mut curr);
            engine.process_audio(&input, &mut curr);
        }

        // The input is periodic, so a steady-state limiter must produce nearly
        // identical consecutive blocks; a per-block normaliser would not.
        let seam = (curr[0] - prev[0]).abs().max((curr[1] - prev[1]).abs());
        let block_delta = curr
            .iter()
            .zip(prev.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);

        println!("Limiter seam={seam:.5} max block delta={block_delta:.5}");
        assert!(
            curr.iter().all(|s| s.is_finite() && s.abs() <= 1.0001),
            "limiter let the signal past the ceiling"
        );
        assert!(
            block_delta < 0.02,
            "limiter gain still jumps between buffers: {block_delta:.5}"
        );
    }

    #[test]
    fn test_visualizer_decays_when_audio_stops() {
        // The FFT is only refreshed on the active path, so the silent and
        // powered-off paths must fade the bars instead of latching them.
        let mut engine = AudioEngine::new();
        let loud = stereo_sine(1000.0, 0.5, 512);
        let mut out = vec![0.0f32; loud.len()];
        engine.process_audio(&loud, &mut out);
        let peak_before = engine.get_fft_data().iter().cloned().fold(0.0f32, f32::max);
        assert!(peak_before > 0.0, "visualizer saw nothing during playback");

        // Playback stops: feed silence.
        let silence = vec![0.0f32; loud.len()];
        for _ in 0..40 {
            engine.process_audio(&silence, &mut out);
        }
        let peak_after = engine.get_fft_data().iter().cloned().fold(0.0f32, f32::max);
        println!("Visualizer peak {peak_before:.2} -> {peak_after:.2} after silence");
        assert_eq!(peak_after, 0.0, "visualizer bars froze instead of decaying");

        // Same again for the power-off path.
        engine.process_audio(&loud, &mut out);
        assert!(engine.get_fft_data().iter().cloned().fold(0.0f32, f32::max) > 0.0);
        engine.set_power(false);
        for _ in 0..40 {
            engine.process_audio(&loud, &mut out);
        }
        assert_eq!(
            engine.get_fft_data().iter().cloned().fold(0.0f32, f32::max),
            0.0,
            "visualizer bars froze after power-off"
        );
    }

    #[test]
    fn test_every_shipped_preset_stays_within_headroom() {
        // Guards the combination that actually reaches users: a preset's EQ
        // curve and its effect values stacked on top of each other. Bass-heavy
        // presets in particular stack a +10 dB low band with HyperBass.
        let mut input = vec![0.0f32; 2048];
        for (i, frame) in input.chunks_exact_mut(2).enumerate() {
            // Broadband, slightly decorrelated so surround and the reverb engage.
            let t = i as f32;
            frame[0] = 0.22 * (t * 0.01).sin() + 0.18 * (t * 0.21).sin() + 0.12 * (t * 0.93).sin();
            frame[1] = 0.22 * (t * 0.01 + 0.5).sin()
                + 0.18 * (t * 0.19).sin()
                + 0.12 * (t * 0.87).sin();
        }

        for (index, name) in PRESET_NAMES.iter().enumerate() {
            let mut engine = AudioEngine::new();
            load_preset(&mut engine, index);
            let out = settle(&mut engine, &input, 8);
            let g = gain_db(&input, &out);

            println!("preset {name:<12} gain {g:+.2} dB");

            assert!(
                out.iter().all(|s| s.is_finite()),
                "preset '{name}' produced non-finite output"
            );
            assert!(
                out.iter().all(|s| s.abs() <= 1.0001),
                "preset '{name}' exceeded the limiter ceiling"
            );
            assert!(
                g > -8.0 && g < 12.0,
                "preset '{name}' has an unusable level change: {g:+.2} dB"
            );
        }
    }
}
