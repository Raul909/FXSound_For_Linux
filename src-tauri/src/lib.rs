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
    if !gain.is_finite() {
        return Err("Invalid gain value".to_string());
    }
    let mut engine = state.audio_engine.lock().unwrap_or_else(|e| e.into_inner());
    engine.set_eq_band(band, gain);
    Ok(())
}

/// Set the intensity (0–100) for a named audio effect.
#[tauri::command]
fn set_effect(state: State<AppState>, effect: String, value: f32) -> Result<(), String> {
    if !value.is_finite() {
        return Err("Invalid effect value".to_string());
    }

    // Validate effect name against an allowlist to prevent memory exhaustion (DoS)
    let valid_effects = ["fidelity", "ambiance", "dynamic", "surround", "bass"];
    if !valid_effects.contains(&effect.as_str()) {
        return Err("Invalid effect name".to_string());
    }

    let mut engine = state.audio_engine.lock().unwrap_or_else(|e| e.into_inner());
    engine.set_effect(&effect, value);
    Ok(())
}

#[derive(serde::Deserialize, Default)]
pub struct PresetEffects {
    fidelity: Option<f32>,
    ambiance: Option<f32>,
    dynamic: Option<f32>,
    surround: Option<f32>,
    bass: Option<f32>,
}

/// Apply a full preset state (EQ bands + effects) in one batch to avoid IPC overhead.
#[tauri::command]
fn apply_preset_state(
    state: State<AppState>,
    eq_bands: [f32; 10],
    effects: PresetEffects,
) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().unwrap_or_else(|e| e.into_inner());

    for (band, &gain) in eq_bands.iter().enumerate() {
        if gain.is_finite() {
            engine.set_eq_band(band, gain);
        }
    }

    if let Some(val) = effects.fidelity {
        if val.is_finite() {
            engine.set_effect("fidelity", val);
        }
    }
    if let Some(val) = effects.ambiance {
        if val.is_finite() {
            engine.set_effect("ambiance", val);
        }
    }
    if let Some(val) = effects.dynamic {
        if val.is_finite() {
            engine.set_effect("dynamic", val);
        }
    }
    if let Some(val) = effects.surround {
        if val.is_finite() {
            engine.set_effect("surround", val);
        }
    }
    if let Some(val) = effects.bass {
        if val.is_finite() {
            engine.set_effect("bass", val);
        }
    }
    Ok(())
}

/// Toggle audio processing on or off.
#[tauri::command]
fn set_power(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut engine = state.audio_engine.lock().unwrap_or_else(|e| e.into_inner());
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
    let engine = state.audio_engine.lock().unwrap_or_else(|e| e.into_inner());
    Ok(engine.get_fft_data())
}

// ── App Initialization ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Work around WebKitGTK failing to bring up its GPU rendering path on some
    // Linux systems (newer Mesa/Wayland — e.g. Fedora + GNOME — as well as
    // NVIDIA drivers and VMs). Symptoms: the webview aborts at startup with
    // "Could not create default EGL display: EGL_BAD_PARAMETER. Aborting..."
    // (a blank AppImage window), or — when the system WebKitGTK renders but its
    // accelerated compositor wedges on the Wayland surface — the window paints
    // once and then goes "Not Responding" (seen on the deb/rpm builds).
    //
    // The real fix is to disable accelerated compositing entirely, so WebKitGTK
    // renders in software and never creates an EGL display. Disabling only the
    // DMABUF renderer (as 1.1.1 did) was not enough — the EGL display is still
    // created for the compositor, so the abort persisted even with
    // WEBKIT_DISABLE_DMABUF_RENDERER=1 set. This UI is a lightweight EQ panel,
    // so software rendering costs nothing perceptible. Respect explicit user
    // overrides so anyone who wants the GPU path back can opt in.
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none() {
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
        // Keep the DMABUF renderer disabled as a second layer (from 1.1.1).
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

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
            apply_preset_state,
            set_power,
            get_audio_devices,
            get_visualizer_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
