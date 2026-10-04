//! Audio processing engine for FXSound.
//!
//! Provides a 10-band equalizer using biquad peak filters, audio effects
//! (fidelity, dynamic compression, bass boost, ambiance reverb, and 3D
//! surround widening), and real-time FFT-based spectrum analysis for the
//! visualizer. Getting audio in and out of the system is `pulse.rs`'s job.

use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::collections::HashMap;
use std::sync::Arc;

pub const SAMPLE_RATE: u32 = 48000;
pub const CHANNELS: u8 = 2;
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

/// Frames over which a biquad glides to new coefficients (20 ms at 48 kHz).
///
/// Measured on a Flat → Deep Bass preset switch: 5 ms let the sudden +12 dB
/// of bass hit the limiter before its gain could follow, 40 ms rose slower
/// than the compressor attacks; 20 ms gave the smallest overshoot.
const COEF_GLIDE_FRAMES: u32 = 960;

/// Second-order IIR (biquad) filter coefficients and state.
///
/// Used for peaking EQ filters — each band gets its own biquad
/// that only boosts/cuts around its center frequency.
#[derive(Clone)]
struct BiquadFilter {
    // Coefficients currently in use
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // Coefficient glide: where it ends, the per-frame increment, frames left
    target: [f32; 5],
    step: [f32; 5],
    glide_left: u32,

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
            target: [b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0],
            step: [0.0; 5],
            glide_left: 0,
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
            target: [b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0],
            step: [0.0; 5],
            glide_left: 0,
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
            target: [1.0, 0.0, 0.0, 0.0, 0.0],
            step: [0.0; 5],
            glide_left: 0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Glide to another filter's coefficients, keeping this filter's delay
    /// line.
    ///
    /// Moving a slider used to replace the whole filter, zeroing its history
    /// mid-stream, so the output restarted from silence — a click on every
    /// step of a drag. Even with the history kept, switching coefficients in
    /// one sample leaves a step in the output on big jumps such as preset
    /// changes, so they move linearly over COEF_GLIDE_FRAMES instead.
    ///
    /// Gliding cannot destabilise the filter: a biquad is stable exactly when
    /// (a1, a2) lies inside the triangle |a2| < 1, |a1| < 1 + a2. That region
    /// is convex, so every point on the line between two stable designs is
    /// stable too.
    fn set_coefficients(&mut self, from: &BiquadFilter) {
        self.target = [from.b0, from.b1, from.b2, from.a1, from.a2];
        let current = [self.b0, self.b1, self.b2, self.a1, self.a2];
        for ((step, &to), &now) in self.step.iter_mut().zip(&self.target).zip(&current) {
            *step = (to - now) / COEF_GLIDE_FRAMES as f32;
        }
        self.glide_left = COEF_GLIDE_FRAMES;
    }

    /// Advance the coefficient glide by one frame, landing exactly on target.
    #[inline]
    fn advance_glide(&mut self) {
        self.glide_left -= 1;
        if self.glide_left == 0 {
            [self.b0, self.b1, self.b2, self.a1, self.a2] = self.target;
        } else {
            self.b0 += self.step[0];
            self.b1 += self.step[1];
            self.b2 += self.step[2];
            self.a1 += self.step[3];
            self.a2 += self.step[4];
        }
    }

    /// Forget all past samples.
    fn reset(&mut self) {
        self.x1 = [0.0; 2];
        self.x2 = [0.0; 2];
        self.y1 = [0.0; 2];
        self.y2 = [0.0; 2];
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

        // Coefficients advance once per stereo frame, after the right channel.
        if ch == 1 && self.glide_left > 0 {
            self.advance_glide();
        }

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

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.filter_store = 0.0;
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

    fn reset(&mut self) {
        self.buffer.fill(0.0);
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

    /// Silence the tail.
    fn reset(&mut self) {
        self.combs_l.iter_mut().for_each(CombFilter::reset);
        self.combs_r.iter_mut().for_each(CombFilter::reset);
        self.allpass_l.iter_mut().for_each(AllpassFilter::reset);
        self.allpass_r.iter_mut().for_each(AllpassFilter::reset);
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

    fn reset(&mut self) {
        self.gain = 1.0;
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

    fn reset(&mut self) {
        self.env = 0.0;
    }

    /// Follow the stereo peak envelope. Runs even while Dynamic Boost is at
    /// zero so that raising the slider starts from the true signal level
    /// instead of a stale one from whenever it was last used.
    #[inline]
    fn track(&mut self, l: f32, r: f32) {
        let peak = l.abs().max(r.abs());
        let coef = if peak > self.env {
            self.attack
        } else {
            self.release
        };
        self.env += (peak - self.env) * coef;
        if self.env < DENORMAL_FLUSH {
            self.env = 0.0;
        }
    }

    /// - `threshold` — linear level above which gain reduction starts
    /// - `slope` — 1/ratio (1.0 = no compression, 0.4 = 2.5:1)
    /// - `makeup` — output gain applied after compression
    #[inline]
    fn process_frame(&mut self, l: &mut f32, r: &mut f32, threshold: f32, slope: f32, makeup: f32) {
        self.track(*l, *r);

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

/// Effect names accepted by [`AudioEngine::set_effect`], as sent by the UI.
pub const EFFECT_NAMES: [&str; 5] = ["fidelity", "ambiance", "dynamic", "surround", "bass"];

/// Length of the bypass crossfade (seconds). Jumping between the processed and
/// the original signal in a single sample is a step in the waveform — a click
/// on every press of the power button — so the two are blended instead.
const BYPASS_FADE_SECONDS: f32 = 0.02;

/// Samples quieter than this count as digital silence (about −120 dBFS).
const SILENCE_THRESHOLD: f32 = 1e-6;

/// How long the input must stay digitally silent before processing pauses.
///
/// The previous gate muted any buffer whose RMS fell below −60 dBFS, which is
/// quiet music, not silence: fade-outs and soft passages were cut to nothing,
/// and the ambiance tail was chopped the moment a track ended. Now only true
/// silence pauses the DSP, and only once every tail has had time to ring out.
const SILENCE_HANGOVER_SECONDS: f32 = 2.0;

/// The visualizer runs one FFT per this many stereo frames.
const FFT_HOP: usize = FFT_SIZE;

/// Values below this are flushed to zero in decaying one-pole states, which
/// would otherwise sink into denormals during silence (slow on x86).
const DENORMAL_FLUSH: f32 = 1e-25;

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

    /// One biquad per EQ band. All ten always run: a flat band is an exact
    /// identity (b0 = 1, every other coefficient 0), and keeping it in the
    /// chain keeps its delay line in step with the signal, so a band that
    /// starts moving never begins from stale state.
    filters: Vec<BiquadFilter>,

    /// FFT magnitude data shared with the UI for the visualizer.
    pub fft_data: Arc<std::sync::Mutex<Vec<f32>>>,

    /// Precomputed FFT bin boundaries for mapping to 32 visualizer bars.
    fft_bin_boundaries: [usize; 33],

    /// Stereo reverb driving the "ambiance" effect (spatial ambience).
    reverb: StereoReverb,

    /// Low-shelf filter implementing HyperBass (identity while at zero).
    bass_shelf: BiquadFilter,

    /// One-pole low-pass state (per channel) used to split off the high band
    /// that the Fidelity exciter saturates.
    fidelity_lp: [f32; 2],
    fidelity_lp_coef: f32,

    /// Dynamics stages, held across buffers so their envelopes stay continuous.
    compressor: Compressor,
    limiter: Limiter,

    /// Effect amounts actually applied (fidelity, ambiance, dynamic, surround
    /// as 0–1), easing towards the slider values instead of jumping: a preset
    /// change could otherwise step the makeup gain or stereo width mid-wave.
    fx: [f32; 4],
    fx_smooth: f32,

    /// Hann window applied before the visualizer FFT to suppress the spectral
    /// leakage that made neighbouring bars bleed into each other.
    fft_window: Vec<f32>,

    /// Position of the bypass crossfade: 1.0 = processed, 0.0 = original.
    mix: f32,
    /// How far `mix` moves per stereo frame while fading.
    mix_step: f32,

    /// Consecutive digitally-silent input frames, and how many are allowed
    /// before processing pauses.
    silent_frames: usize,
    silence_hangover: usize,

    /// The most recent FFT_SIZE mono samples, oldest at `fft_ring_pos`.
    ///
    /// The FFT used to run only on calls carrying at least 1024 samples, so
    /// any capture fragment smaller than that left the visualizer frozen.
    /// Accumulating here makes it independent of how audio is chunked.
    fft_ring: Vec<f32>,
    fft_ring_pos: usize,
    /// Frames gathered since the last FFT, and since the last decay step.
    fft_pending: usize,
    decay_pending: usize,
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
            .map(|n| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * n as f32 / FFT_SIZE as f32).cos()))
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
            fx: [0.0; 4],
            fx_smooth: time_coef(20e-3, sample_rate),
            fft_window,
            mix: 1.0,
            mix_step: 1.0 / (BYPASS_FADE_SECONDS * sample_rate),
            silent_frames: 0,
            silence_hangover: (SILENCE_HANGOVER_SECONDS * sample_rate) as usize,
            fft_ring: vec![0.0; FFT_SIZE],
            fft_ring_pos: 0,
            fft_pending: 0,
            decay_pending: 0,
        }
    }

    /// Set the gain for a single EQ band, keeping the filter's state.
    pub fn set_eq_band(&mut self, band: usize, gain: f32) {
        if band >= EQ_FREQUENCIES.len() || !gain.is_finite() {
            return;
        }
        self.eq_bands[band] = gain.clamp(-12.0, 12.0);

        // Q factor of 1.4 gives a moderate bandwidth suitable for a 10-band EQ
        let design = if self.eq_bands[band].abs() < 0.1 {
            BiquadFilter::flat()
        } else {
            BiquadFilter::peaking_eq(
                EQ_FREQUENCIES[band],
                self.eq_bands[band],
                1.4,
                self.sample_rate as f32,
            )
        };
        self.filters[band].set_coefficients(&design);
        log::debug!("EQ band {} set to {:.1} dB", band, self.eq_bands[band]);
    }

    /// Set an effect intensity value (0–100).
    pub fn set_effect(&mut self, effect: &str, value: f32) {
        if !value.is_finite() {
            return;
        }
        let clamped = value.clamp(0.0, 100.0);
        self.effects.insert(effect.to_string(), clamped);

        // HyperBass runs through a low-shelf biquad, so its coefficients have
        // to be recomputed whenever the slider moves.
        if effect == "bass" {
            let design = if clamped < 0.5 {
                BiquadFilter::flat()
            } else {
                BiquadFilter::low_shelf(
                    BASS_SHELF_FREQ,
                    (clamped / 100.0) * BASS_SHELF_MAX_DB,
                    0.9,
                    self.sample_rate as f32,
                )
            };
            self.bass_shelf.set_coefficients(&design);
        }

        log::debug!("Effect '{}' set to {:.1}", effect, clamped);
    }

    /// Toggle processing. Off is a true bypass: the original audio passes
    /// through unchanged. Both directions crossfade over a few milliseconds.
    pub fn set_power(&mut self, enabled: bool) {
        self.powered = enabled;
        log::info!("Power: {}", if enabled { "ON" } else { "OFF (bypass)" });
    }

    /// Return the current FFT magnitude data for the visualizer (32 bins).
    pub fn get_fft_data(&self) -> Vec<f32> {
        self.fft_data
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    // ── Main processing pipeline ──

    /// Process interleaved stereo audio: EQ, effects and limiter, with a
    /// crossfaded bypass, then feed the visualizer.
    ///
    /// Works on any number of whole frames, so it does not care how the
    /// audio server chunks the stream.
    pub fn process_audio(&mut self, input: &[f32], output: &mut [f32]) {
        let ch = CHANNELS as usize;
        let len = input.len().min(output.len()) / ch * ch;
        let (input, output) = (&input[..len], &mut output[..len]);
        let frames = len / ch;
        if frames == 0 {
            return;
        }

        let target = if self.powered { 1.0 } else { 0.0 };

        // Fully bypassed: hand the original audio straight through. Audio
        // reaches the speakers only via FXSound now, so this must never be
        // silence — the old engine muted here because, back then, the
        // unprocessed audio was also playing directly.
        if self.mix <= 0.0 && target <= 0.0 {
            output.copy_from_slice(input);
            self.decay_fft(frames);
            return;
        }

        if input.iter().all(|s| s.abs() < SILENCE_THRESHOLD) {
            self.silent_frames = self.silent_frames.saturating_add(frames);
        } else {
            self.silent_frames = 0;
        }
        if self.silent_frames > self.silence_hangover {
            if self.silent_frames - frames <= self.silence_hangover {
                // Just went quiet for good: every tail has rung out, so let
                // the next sound start from clean state.
                self.reset_state();
            }
            output.fill(0.0);
            self.mix = target; // nothing audible left to crossfade
            self.decay_fft(frames);
            return;
        }

        // Re-entering from a full bypass: drop state from before power went
        // off, such as the reverb tail of whatever was playing then.
        if self.mix <= 0.0 {
            self.reset_state();
        }

        self.apply_eq(input, output);
        self.apply_effects(output);
        self.apply_limiter(output);

        if self.mix != target {
            for (out, dry) in output.chunks_exact_mut(ch).zip(input.chunks_exact(ch)) {
                self.mix = if target > self.mix {
                    (self.mix + self.mix_step).min(target)
                } else {
                    (self.mix - self.mix_step).max(target)
                };
                out[0] = dry[0] + (out[0] - dry[0]) * self.mix;
                out[1] = dry[1] + (out[1] - dry[1]) * self.mix;
            }
        }

        if self.powered {
            self.update_fft(output);
        } else {
            // Let the bars fall while fading out to bypass.
            self.decay_fft(frames);
        }
    }

    /// Clear every filter, envelope and reverb buffer.
    fn reset_state(&mut self) {
        self.filters.iter_mut().for_each(BiquadFilter::reset);
        self.bass_shelf.reset();
        self.fidelity_lp = [0.0; 2];
        self.reverb.reset();
        self.compressor.reset();
        self.limiter.reset();
    }

    // ── EQ Processing ──

    /// Apply all 10 biquad EQ filters in series to the audio buffer.
    ///
    /// Each filter only affects frequencies around its center frequency,
    /// so adjusting the 32 Hz band won't change treble, and vice versa.
    fn apply_eq(&mut self, input: &[f32], output: &mut [f32]) {
        // Interleaved stereo: chunk[0] = left, chunk[1] = right
        for (out, inp) in output
            .chunks_exact_mut(CHANNELS as usize)
            .zip(input.chunks_exact(CHANNELS as usize))
        {
            let mut l = inp[0];
            let mut r = inp[1];
            for filter in self.filters.iter_mut() {
                l = filter.process(l, 0);
                r = filter.process(r, 1);
            }
            out[0] = l;
            out[1] = r;
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
    ///
    /// Every stateful stage keeps running at zero (contributing nothing), so
    /// raising a slider from zero picks up from the live signal instead of
    /// whatever its filters held when it was last switched off.
    fn apply_effects(&mut self, buffer: &mut [f32]) {
        let target = [
            self.effects.get("fidelity").copied().unwrap_or(0.0) / 100.0,
            self.effects.get("ambiance").copied().unwrap_or(0.0) / 100.0,
            self.effects.get("dynamic").copied().unwrap_or(0.0) / 100.0,
            self.effects.get("surround").copied().unwrap_or(0.0) / 100.0,
        ];
        let k = self.fx_smooth;
        let lp_coef = self.fidelity_lp_coef;

        for frame in buffer.chunks_exact_mut(CHANNELS as usize) {
            for (now, &to) in self.fx.iter_mut().zip(&target) {
                *now += (to - *now) * k;
                if (to - *now).abs() < 1e-5 {
                    *now = to;
                }
            }
            let [fidelity, ambiance, dynamic, surround] = self.fx;
            let (mut l, mut r) = (frame[0], frame[1]);

            // ── HyperBass: low-shelf boost below ~110 Hz (identity at zero) ──
            l = self.bass_shelf.process(l, 0);
            r = self.bass_shelf.process(r, 1);

            // ── Fidelity: high-band harmonic exciter ──
            // Splits off the band above ~3 kHz, drives only that, and adds it
            // back on top of the untouched dry signal.
            let drive = 1.5 + fidelity * 2.5;
            let mix = fidelity * 0.30;
            for (sample, lp) in [&mut l, &mut r]
                .into_iter()
                .zip(self.fidelity_lp.iter_mut())
            {
                *lp += (*sample - *lp) * lp_coef;
                if lp.abs() < DENORMAL_FLUSH {
                    *lp = 0.0;
                }
                if mix > 0.0 {
                    let high = *sample - *lp;
                    *sample += (high * drive).tanh() * mix;
                }
            }

            // ── 3D Surround: mid/side stereo widening ──
            // Width runs from 1.0 (no change) at 0 to 2.0 at 100. The mid
            // (mono) component is preserved, so mono content and downmix
            // compatibility are unaffected — only the stereo "side" widens.
            if surround > 0.0 {
                let mid = (l + r) * 0.5;
                let side = (l - r) * 0.5 * (1.0 + surround);
                l = mid + side;
                r = mid - side;
            }

            // ── Ambiance: stereo reverb mixed on top of the dry signal ──
            // A parallel "send": the dry signal stays intact and scaled wet
            // reverb is added, so ambiance adds space without hollowing out
            // the original. The limiter downstream tames peaks.
            let (wet_l, wet_r) = self.reverb.process(l, r);
            let wet = ambiance * 0.45;
            l += wet_l * wet;
            r += wet_r * wet;

            // ── Dynamic Boost: compression with makeup gain ──
            // Narrows the gap between quiet and loud passages, then gives back
            // the headroom compression removed, so the result is audibly
            // louder and denser — which is what the slider name promises.
            if dynamic > 0.0 {
                let threshold = 1.0 - 0.45 * dynamic;
                let slope = 1.0 - 0.6 * dynamic;
                // Gain that restores a full-scale peak back to full scale.
                let makeup = 1.0 / (threshold + (1.0 - threshold) * slope);
                self.compressor
                    .process_frame(&mut l, &mut r, threshold, slope, makeup);
            } else {
                self.compressor.track(l, r);
            }

            frame[0] = l;
            frame[1] = r;
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

    /// Fade the visualizer bars towards zero by one step per FFT_HOP frames.
    ///
    /// Used on the silent and bypassed paths, which never reach the FFT.
    /// Without it the last magnitudes stayed latched and the bars froze
    /// mid-height whenever playback stopped.
    fn decay_fft(&mut self, frames: usize) {
        self.fft_pending = 0;
        self.decay_pending += frames;
        if self.decay_pending < FFT_HOP {
            return;
        }
        let steps = (self.decay_pending / FFT_HOP).min(64) as i32;
        self.decay_pending %= FFT_HOP;
        let factor = 0.75f32.powi(steps);

        let mut fft_data = self.fft_data.lock().unwrap_or_else(|e| e.into_inner());
        for value in fft_data.iter_mut() {
            *value *= factor;
            if *value < 0.01 {
                *value = 0.0;
            }
        }
    }

    /// Feed processed audio to the visualizer, running an FFT every FFT_HOP
    /// frames regardless of how the audio arrives.
    fn update_fft(&mut self, buffer: &[f32]) {
        self.decay_pending = 0;
        for frame in buffer.chunks_exact(CHANNELS as usize) {
            self.fft_ring[self.fft_ring_pos] = (frame[0] + frame[1]) * 0.5;
            self.fft_ring_pos = (self.fft_ring_pos + 1) % FFT_SIZE;
            self.fft_pending += 1;
            if self.fft_pending >= FFT_HOP {
                self.fft_pending = 0;
                self.compute_fft();
            }
        }
    }

    /// Windowed FFT of the ring buffer, mapped onto the 32 visualizer bars.
    fn compute_fft(&mut self) {
        // The ring's write position holds its oldest sample. Without a window
        // the abrupt block edges smear energy across every bin, so a pure tone
        // lit up bars either side of it.
        for (i, (complex, w)) in self
            .complex_buffer
            .iter_mut()
            .zip(self.fft_window.iter())
            .enumerate()
        {
            let sample = self.fft_ring[(self.fft_ring_pos + i) % FFT_SIZE];
            *complex = Complex::new(sample * w, 0.0);
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

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
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

    /// Largest sample-to-sample step of the left channel in `buf[from..to]`
    /// (interleaved frames).
    fn max_step(buf: &[f32], from: usize, to: usize) -> f32 {
        buf.chunks_exact(2)
            .skip(from.max(1) - 1)
            .take(to - from.max(1) + 1)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| (w[1][0] - w[0][0]).abs())
            .fold(0.0f32, f32::max)
    }

    /// Run `input` through `engine` in fragments of `frag` frames, calling
    /// `at(frame_index, engine)` before each fragment.
    fn run_fragments(
        engine: &mut AudioEngine,
        input: &[f32],
        frag: usize,
        mut at: impl FnMut(usize, &mut AudioEngine),
    ) -> Vec<f32> {
        let mut out = vec![0.0f32; input.len()];
        for (i, (inp, o)) in input
            .chunks(frag * 2)
            .zip(out.chunks_mut(frag * 2))
            .enumerate()
        {
            at(i * frag, engine);
            engine.process_audio(inp, o);
        }
        out
    }

    #[test]
    fn test_eq_change_mid_stream_is_click_free() {
        // A slider move used to rebuild the band's filter from scratch, zeroing
        // its history mid-stream, and even keeping the history a one-sample
        // coefficient switch leaves a step on big jumps (preset changes). A
        // tone right on the band's centre, cut from +6 dB to -6 dB at once, is
        // the worst case: the output must change level without a step.
        // The 32 Hz band with a tone on its centre: bass is where a step is
        // most audible, because the waveform itself moves so little per
        // sample. 31.86 Hz puts a peak exactly on the change; at a zero
        // crossing even a full state reset would leave no visible step.
        let input = stereo_sine(31.86, 0.3, 48_000);
        let mut engine = AudioEngine::new();
        engine.set_eq_band(0, 6.0);
        let change_at = 24_480;
        let out = run_fragments(&mut engine, &input, 480, |frame, e| {
            if frame == change_at {
                e.set_eq_band(0, -6.0);
            }
        });

        let around = max_step(&out, change_at - 480, change_at + 2_400);
        let before = max_step(&out, change_at - 9_600, change_at - 480);
        let after = max_step(&out, change_at + 9_600, change_at + 19_200);
        println!(
            "EQ +6 -> -6 dB: step around change {around:.4}, before {before:.4}, after {after:.4}"
        );
        assert!(
            around <= before.max(after) * 1.1,
            "EQ change produced a discontinuity: {around:.4} vs {:.4}",
            before.max(after)
        );
    }

    #[test]
    fn test_power_toggle_crossfades_and_bypass_is_exact() {
        // Off is now a true bypass (the original audio, unchanged), reached
        // through a short crossfade instead of a hard switch. Bass Boost on a
        // 100 Hz tone makes processed and original very different, so a hard
        // switch would show up as a large step.
        let input = stereo_sine(100.49, 0.4, 48_000); // a peak lands on the toggle
        let mut engine = AudioEngine::new();
        load_preset(&mut engine, 5); // Bass Boost
        let toggle_at = 24_480;
        let out = run_fragments(&mut engine, &input, 480, |frame, e| {
            if frame == toggle_at {
                e.set_power(false);
            }
        });

        let around = max_step(&out, toggle_at - 480, toggle_at + 2_400);
        let wet = max_step(&out, toggle_at - 9_600, toggle_at - 480);
        let dry = max_step(&input, toggle_at + 4_800, toggle_at + 9_600);
        println!("power toggle: step {around:.4}, processed {wet:.4}, original {dry:.4}");
        assert!(
            around <= wet.max(dry) * 1.1,
            "power toggle clicked: {around:.4} vs {:.4}",
            wet.max(dry)
        );

        // 20 ms later the output must be the input, bit for bit.
        let settled = (toggle_at + 1_920) * 2;
        assert_eq!(
            &out[settled..],
            &input[settled..],
            "bypass altered the audio"
        );
    }

    #[test]
    fn test_quiet_passages_are_not_muted() {
        // The old gate zeroed anything under -60 dBFS RMS, so fade-outs and
        // soft passages vanished. Only true digital silence may be skipped.
        let input = stereo_sine(440.0, 3e-4, 4_800); // about -73 dBFS RMS
        let mut engine = AudioEngine::new();
        let out = settle(&mut engine, &input, 4);
        let g = gain_db(&input, &out);
        println!("quiet passage gain: {g:+.2} dB");
        assert!(
            g.abs() < 0.1,
            "quiet audio was altered or muted: {g:+.2} dB"
        );
    }

    #[test]
    fn test_visualizer_runs_on_small_fragments() {
        // Capture now arrives in ~10 ms fragments; the FFT used to need 1024
        // samples in a single call and would never have run.
        let input = stereo_sine(1000.0, 0.5, 4_800);
        let mut engine = AudioEngine::new();
        run_fragments(&mut engine, &input, 480, |_, _| {});
        let peak = engine.get_fft_data().iter().cloned().fold(0.0f32, f32::max);
        assert!(peak > 10.0, "visualizer idle on small fragments: {peak:.2}");
    }

    #[test]
    fn test_reverb_tail_survives_the_end_of_input() {
        // The silence gate must not chop the ambiance tail the instant a
        // track ends, but must stop processing once silence is real.
        let mut engine = AudioEngine::new();
        engine.set_effect("ambiance", 100.0);
        let burst = stereo_sine(500.0, 0.4, 24_000);
        settle(&mut engine, &burst, 1);

        let silence = vec![0.0f32; 9_600]; // 100 ms
        let mut out = vec![0.0f32; silence.len()];
        engine.process_audio(&silence, &mut out);
        let tail = rms(&out);
        for _ in 0..30 {
            engine.process_audio(&silence, &mut out); // 3 more seconds
        }
        println!("reverb tail RMS right after input stops: {tail:.5}");
        assert!(tail > 1e-3, "reverb tail was cut off");
        assert!(
            out.iter().all(|&s| s == 0.0),
            "output not silent after the hangover"
        );
    }

    #[test]
    fn test_odd_or_mismatched_buffers_are_handled() {
        let mut engine = AudioEngine::new();
        let input = vec![0.1f32; 7];
        let mut short = vec![0.0f32; 4];
        engine.process_audio(&input, &mut short); // must not panic
        let mut empty: Vec<f32> = Vec::new();
        engine.process_audio(&input, &mut empty);
        assert!(short.iter().all(|s| s.is_finite()));
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
            frame[1] =
                0.22 * (t * 0.01 + 0.5).sin() + 0.18 * (t * 0.19).sin() + 0.12 * (t * 0.87).sin();
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
