//! FXSound Tauri application entry point and command handlers.
//!
//! Sets up the audio engine, registers Tauri commands that the React
//! frontend can call via `invoke()`, and starts the PulseAudio processor.

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

mod audio;
use audio::{AudioEngine, AudioProcessor};

/// Shared application state holding the audio engine behind a mutex.
struct AppState {
    audio_engine: Arc<Mutex<AudioEngine>>,
}

// ── Tauri Commands ──
// These functions are callable from the frontend via `invoke("command_name", { args })`.

/// Set the gain (in dB) for a single EQ band.
#[tauri::command]
fn set_eq_band(state: State<AppState>, band: usize, gain: f32) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_eq_band(band, gain);
    Ok(())
}

/// Set the intensity (0–100) for a named audio effect.
#[tauri::command]
fn set_effect(state: State<AppState>, effect: String, value: f32) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_effect(&effect, value);
    Ok(())
}

/// Toggle audio processing on or off.
#[tauri::command]
fn set_power(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_power(enabled);
    Ok(())
}

/// Return the list of available audio output devices by querying PulseAudio.
#[tauri::command]
fn get_audio_devices() -> Result<Vec<String>, String> {
    audio::get_pulse_sinks().map_err(|e| format!("Failed to get audio devices: {}", e))
}

/// Return the current FFT magnitude data for the visualizer (32 bins).
#[tauri::command]
fn get_visualizer_data(state: State<AppState>) -> Result<Vec<f32>, String> {
    let engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_fft_data())
}

// ── App Initialization ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Enable debug logging in development builds
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Create the shared audio engine
            let audio_engine = Arc::new(Mutex::new(AudioEngine::new()));

            // Start the PulseAudio capture → process → playback loop
            let processor = AudioProcessor::new(Arc::clone(&audio_engine));
            if let Err(e) = processor.start() {
                log::error!("Failed to start audio processor: {}", e);
            } else {
                log::info!("Audio processor started successfully");
            }

            // Store state so Tauri commands can access the engine
            app.manage(AppState { audio_engine });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_eq_band,
            set_effect,
            set_power,
            get_audio_devices,
            get_visualizer_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
