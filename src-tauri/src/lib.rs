//! FXSound Tauri application entry point and command handlers.
//!
//! Sets up the audio engine, registers Tauri commands that the React
//! frontend can call via `invoke()`, and starts the PulseAudio processor.

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

mod audio;
use audio::{AudioEngine, AudioProcessor, AudioSink, OutputRouting};

/// Shared application state holding the audio engine behind a mutex.
struct AppState {
    audio_engine: Arc<Mutex<AudioEngine>>,
    /// Lets `set_output_device` retarget the running playback stream.
    routing: OutputRouting,
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
fn get_audio_devices() -> Result<Vec<AudioSink>, String> {
    audio::get_pulse_sinks().map_err(|e| format!("Failed to get audio devices: {}", e))
}

/// Route processed audio to a specific sink, or to the system default when
/// `sink` is `None`.
#[tauri::command]
fn set_output_device(state: State<AppState>, sink: Option<String>) -> Result<(), String> {
    // An empty string is the frontend's "system default" sentinel.
    let sink = sink.filter(|s| !s.is_empty());
    state.routing.set_sink(sink);
    Ok(())
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
    // NVIDIA drivers and VMs like VirtualBox/VMware). Symptoms: the webview
    // aborts at startup with
    // "Could not create default EGL display: EGL_BAD_PARAMETER. Aborting..."
    // (a blank AppImage window), or — when the system WebKitGTK renders but its
    // accelerated compositor wedges on the Wayland surface — the window paints
    // once and then goes "Not Responding" (seen on the deb/rpm builds).
    //
    // The 1.1.1/1.1.2 fixes (disable the DMABUF renderer, then disable
    // accelerated compositing) were both incomplete: WebKitGTK still brings up a
    // *shared* EGL display at process startup regardless of compositing mode, so
    // when the GPU stack can't hand one back the hard abort survived. The
    // definitive fix is to point Mesa at its software rasteriser (llvmpipe) via
    // LIBGL_ALWAYS_SOFTWARE — that EGL display then always succeeds. This is the
    // very path that makes the system-installed .deb work on the same VM where
    // the bundled AppImage aborts, so we apply it for every package type.
    //
    // This UI is a lightweight EQ panel, so software rendering costs nothing
    // perceptible. We respect explicit user overrides so anyone who wants the
    // GPU path back can opt in (e.g. LIBGL_ALWAYS_SOFTWARE=0).
    #[cfg(target_os = "linux")]
    {
        // Primary fix (1.1.3): force Mesa software rendering so WebKitGTK's
        // startup EGL display can always be created, even with no working GPU.
        if std::env::var_os("LIBGL_ALWAYS_SOFTWARE").is_none() {
            std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
        }
        // Disable accelerated compositing so web content never needs the GPU
        // path (from 1.1.2).
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
            let routing = OutputRouting::default();

            // Start the PulseAudio capture → process → playback loop
            let processor = AudioProcessor::new(Arc::clone(&audio_engine), routing.clone());
            if let Err(e) = processor.start() {
                log::error!("Failed to start audio processor: {}", e);
            } else {
                log::info!("Audio processor started successfully");
            }

            // Store state so Tauri commands can access the engine
            app.manage(AppState {
                audio_engine,
                routing,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_eq_band,
            set_effect,
            apply_preset_state,
            set_power,
            get_audio_devices,
            set_output_device,
            get_visualizer_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
