use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

mod audio;
use audio::{AudioEngine, AudioProcessor};

struct AppState {
    audio_engine: Arc<Mutex<AudioEngine>>,
}

#[tauri::command]
fn set_eq_band(state: State<AppState>, band: usize, gain: f32) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_eq_band(band, gain);
    Ok(())
}

#[tauri::command]
fn set_effect(state: State<AppState>, effect: String, value: f32) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_effect(&effect, value);
    Ok(())
}

#[tauri::command]
fn set_power(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    engine.set_power(enabled);
    Ok(())
}

#[tauri::command]
fn get_audio_devices() -> Result<Vec<String>, String> {
    Ok(vec![
        "Built-in Speakers".to_string(),
        "Headphones (3.5mm)".to_string(),
        "USB Audio Device".to_string(),
        "HDMI Audio".to_string(),
        "Bluetooth Headset".to_string(),
    ])
}

#[tauri::command]
fn get_visualizer_data(state: State<AppState>) -> Result<Vec<f32>, String> {
    let engine = state.audio_engine.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_fft_data())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Create audio engine
            let audio_engine = Arc::new(Mutex::new(AudioEngine::new()));
            
            // Start audio processor
            let processor = AudioProcessor::new(Arc::clone(&audio_engine));
            if let Err(e) = processor.start() {
                log::error!("Failed to start audio processor: {}", e);
            } else {
                log::info!("Audio processor started successfully");
            }

            // Store state
            app.manage(AppState { audio_engine });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_eq_band,
            set_effect,
            set_power,
            get_audio_devices,
            get_visualizer_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
