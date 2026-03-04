use std::collections::HashMap;
use std::sync::Arc;
use libpulse_binding as pulse;
use libpulse_simple_binding as psimple;
use rustfft::{FftPlanner, num_complex::Complex};

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: u8 = 2;
const FFT_SIZE: usize = 512;

pub struct AudioEngine {
    powered: bool,
    eq_bands: [f32; 10],
    effects: HashMap<String, f32>,
    sample_rate: u32,
    // Visualizer data
    pub fft_data: Arc<std::sync::Mutex<Vec<f32>>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            powered: true,
            eq_bands: [0.0; 10],
            effects: HashMap::new(),
            sample_rate: SAMPLE_RATE,
            fft_data: Arc::new(std::sync::Mutex::new(vec![0.0; 32])),
        }
    }

    pub fn set_eq_band(&mut self, band: usize, gain: f32) {
        if band < 10 {
            self.eq_bands[band] = gain.clamp(-12.0, 12.0);
            log::info!("EQ Band {} set to {:.1} dB", band, self.eq_bands[band]);
        }
    }

    pub fn set_effect(&mut self, effect: &str, value: f32) {
        let clamped = value.clamp(0.0, 100.0);
        self.effects.insert(effect.to_string(), clamped);
        log::info!("Effect {} set to {:.1}", effect, clamped);
    }

    pub fn set_power(&mut self, enabled: bool) {
        self.powered = enabled;
        log::info!("Power: {}", if enabled { "ON" } else { "OFF" });
    }

    pub fn get_fft_data(&self) -> Vec<f32> {
        self.fft_data.lock().unwrap().clone()
    }

    // Process audio buffer with EQ and effects
    pub fn process_audio(&self, input: &[f32], output: &mut [f32]) {
        if !self.powered {
            output.copy_from_slice(input);
            return;
        }

        self.apply_eq(input, output);
        self.apply_effects(output);
        self.update_fft(output);
    }

    fn apply_eq(&self, input: &[f32], output: &mut [f32]) {
        let frequencies = [32.0, 64.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0];
        output.copy_from_slice(input);
        
        for (i, &gain_db) in self.eq_bands.iter().enumerate() {
            if gain_db.abs() < 0.1 {
                continue;
            }
            
            let gain_linear = db_to_linear(gain_db);
            let freq = frequencies[i];
            
            for sample in output.iter_mut() {
                *sample *= 1.0 + (gain_linear - 1.0) * self.band_weight(freq);
            }
        }
    }

    fn apply_effects(&self, buffer: &mut [f32]) {
        if let Some(&fidelity) = self.effects.get("fidelity") {
            if fidelity > 0.0 {
                let boost = 1.0 + (fidelity / 100.0) * 0.15;
                for sample in buffer.iter_mut() {
                    *sample *= boost;
                }
            }
        }

        if let Some(&dynamic) = self.effects.get("dynamic") {
            if dynamic > 0.0 {
                let threshold = 0.7 - (dynamic / 100.0) * 0.3;
                for sample in buffer.iter_mut() {
                    if sample.abs() > threshold {
                        let sign = sample.signum();
                        *sample = sign * (threshold + (sample.abs() - threshold) * 0.5);
                    }
                }
            }
        }

        if let Some(&bass) = self.effects.get("bass") {
            if bass > 0.0 {
                let boost = 1.0 + (bass / 100.0) * 0.3;
                for sample in buffer.iter_mut() {
                    *sample *= boost;
                }
            }
        }

        let max = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        if max > 1.0 {
            for sample in buffer.iter_mut() {
                *sample /= max;
            }
        }
    }

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

        let magnitudes: Vec<f32> = complex_input[..32]
            .iter()
            .map(|c| (c.norm() * 100.0).min(100.0))
            .collect();

        if let Ok(mut fft_data) = self.fft_data.lock() {
            *fft_data = magnitudes;
        }
    }

    fn band_weight(&self, freq: f32) -> f32 {
        match freq {
            f if f < 100.0 => 0.8,
            f if f < 500.0 => 0.9,
            f if f < 2000.0 => 1.0,
            f if f < 8000.0 => 0.95,
            _ => 0.85,
        }
    }
}

fn db_to_linear(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}

// PulseAudio integration
pub struct AudioProcessor {
    engine: Arc<std::sync::Mutex<AudioEngine>>,
}

impl AudioProcessor {
    pub fn new(engine: Arc<std::sync::Mutex<AudioEngine>>) -> Self {
        Self { engine }
    }

    pub fn start(&self) -> Result<(), String> {
        log::info!("Starting PulseAudio processor...");

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

    fn audio_loop(
        engine: Arc<std::sync::Mutex<AudioEngine>>,
        spec: pulse::sample::Spec,
    ) -> Result<(), String> {
        let input = psimple::Simple::new(
            None,
            "FXSound",
            pulse::stream::Direction::Record,
            None,
            "Audio Input",
            &spec,
            None,
            None,
        ).map_err(|e| format!("Failed to create input: {}", e))?;

        let output = psimple::Simple::new(
            None,
            "FXSound",
            pulse::stream::Direction::Playback,
            None,
            "Audio Output",
            &spec,
            None,
            None,
        ).map_err(|e| format!("Failed to create output: {}", e))?;

        log::info!("PulseAudio streams created successfully");

        const BUFFER_SIZE: usize = 1024;
        let mut input_buffer = vec![0u8; BUFFER_SIZE * 4]; // 4 bytes per f32
        let mut output_buffer = vec![0f32; BUFFER_SIZE];

        loop {
            if let Err(e) = input.read(&mut input_buffer) {
                log::error!("Read error: {}", e);
                break;
            }

            // Convert bytes to f32
            let samples: Vec<f32> = input_buffer
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();

            {
                let engine = engine.lock().unwrap();
                engine.process_audio(&samples, &mut output_buffer);
            }

            // Convert f32 to bytes
            let output_bytes: Vec<u8> = output_buffer
                .iter()
                .flat_map(|&f| f.to_le_bytes())
                .collect();

            if let Err(e) = output.write(&output_bytes) {
                log::error!("Write error: {}", e);
                break;
            }
        }

        Ok(())
    }
}
