//! FXSound Tauri application entry point and command handlers.
//!
//! Wires the audio engine, the system audio routing (`pulse.rs`) and the saved
//! settings to the React frontend, and makes sure the system's own audio
//! routing comes back however the app exits.

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use tauri::{Emitter, Manager, RunEvent, State};

mod audio;
mod instance;
mod pulse;
mod settings;

use audio::{AudioEngine, EFFECT_NAMES};
use pulse::{AudioService, RoutingStatus};
use settings::{Settings, SettingsStore};

/// Shared application state behind the Tauri commands.
struct AppState {
    engine: Arc<Mutex<AudioEngine>>,
    audio: AudioService,
    settings: Arc<SettingsStore>,
}

impl AppState {
    fn engine(&self) -> MutexGuard<'_, AudioEngine> {
        self.engine.lock().unwrap_or_else(|e| e.into_inner())
    }
}

// ── Tauri Commands ──
// These functions are callable from the frontend via `invoke("command_name", { args })`.
// None of them talks to the audio server: synchronous commands run on the GTK
// main thread, and a blocking audio-server query there froze the window (#510).

/// Set the gain (in dB) for a single EQ band.
#[tauri::command]
fn set_eq_band(state: State<AppState>, band: usize, gain: f32) -> Result<(), String> {
    if band >= 10 {
        return Err("Invalid EQ band".to_string());
    }
    if !gain.is_finite() {
        return Err("Invalid gain value".to_string());
    }
    let gain = gain.clamp(-12.0, 12.0);
    state.engine().set_eq_band(band, gain);
    state.settings.update(|s| {
        s.eq[band] = gain;
        s.preset = "Custom".into();
    });
    Ok(())
}

/// Set the intensity (0–100) for a named audio effect.
#[tauri::command]
fn set_effect(state: State<AppState>, effect: String, value: f32) -> Result<(), String> {
    if !value.is_finite() {
        return Err("Invalid effect value".to_string());
    }
    // Validate effect name against an allowlist to prevent memory exhaustion (DoS)
    if !EFFECT_NAMES.contains(&effect.as_str()) {
        return Err("Invalid effect name".to_string());
    }
    let value = value.clamp(0.0, 100.0);
    state.engine().set_effect(&effect, value);
    state.settings.update(|s| {
        s.effects.set(&effect, value);
        s.preset = "Custom".into();
    });
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
    preset: Option<String>,
) -> Result<(), String> {
    let eq = eq_bands.map(|g| {
        if g.is_finite() {
            g.clamp(-12.0, 12.0)
        } else {
            0.0
        }
    });
    let effects = [
        ("fidelity", effects.fidelity),
        ("ambiance", effects.ambiance),
        ("dynamic", effects.dynamic),
        ("surround", effects.surround),
        ("bass", effects.bass),
    ]
    .map(|(name, value)| {
        (
            name,
            value.filter(|v| v.is_finite()).map(|v| v.clamp(0.0, 100.0)),
        )
    });

    {
        let mut engine = state.engine();
        for (band, &gain) in eq.iter().enumerate() {
            engine.set_eq_band(band, gain);
        }
        for (name, value) in effects {
            if let Some(value) = value {
                engine.set_effect(name, value);
            }
        }
    }

    // A preset name is display text only; keep it short and printable.
    let preset = preset
        .filter(|p| !p.is_empty() && p.len() <= 64 && !p.chars().any(char::is_control))
        .unwrap_or_else(|| "Custom".into());
    state.settings.update(|s| {
        s.eq = eq;
        for (name, value) in effects {
            if let Some(value) = value {
                s.effects.set(name, value);
            }
        }
        s.preset = preset;
    });
    Ok(())
}

/// Toggle processing. Off is a bypass: audio still plays, unprocessed.
#[tauri::command]
fn set_power(state: State<AppState>, enabled: bool) -> Result<(), String> {
    state.engine().set_power(enabled);
    state.settings.update(|s| s.powered = enabled);
    Ok(())
}

/// The saved settings, or `None` on first launch.
#[tauri::command]
fn get_settings(state: State<AppState>) -> Option<Settings> {
    state.settings.get()
}

/// Where audio is going and which devices it could go to.
#[tauri::command]
fn get_audio_status(state: State<AppState>) -> RoutingStatus {
    state.audio.status()
}

/// Play to a specific output device, or let FXSound choose when `sink` is
/// `None` or empty.
#[tauri::command]
fn set_output_device(state: State<AppState>, sink: Option<String>) -> Result<(), String> {
    let sink = sink.filter(|s| !s.is_empty());
    state.settings.update(|s| s.output_device = sink.clone());
    state.audio.set_output(sink);
    Ok(())
}

/// Return the current FFT magnitude data for the visualizer (32 bins).
#[tauri::command]
fn get_visualizer_data(state: State<AppState>) -> Result<Vec<f32>, String> {
    Ok(state.engine().get_fft_data())
}

// ── App Initialization ──

/// Exit cleanly on SIGTERM, SIGINT and SIGHUP (logout, Ctrl+C, `kill`), so the
/// FXSound device is removed and the system's own output restored. A second
/// signal, or a shutdown that hangs, exits immediately.
fn exit_on_signals(handle: tauri::AppHandle) {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    let mut signals = match signal_hook::iterator::Signals::new([SIGTERM, SIGINT, SIGHUP]) {
        Ok(signals) => signals,
        Err(e) => {
            log::warn!("Could not install signal handlers: {e}");
            return;
        }
    };
    let spawned = std::thread::Builder::new()
        .name("fxsound-signals".into())
        .spawn(move || {
            let mut signals = signals.forever();
            if let Some(signal) = signals.next() {
                log::info!("Received signal {signal}; restoring audio routing and exiting");
                handle.exit(0);
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_secs(5));
                    std::process::exit(1);
                });
            }
            if signals.next().is_some() {
                std::process::exit(1);
            }
        });
    if let Err(e) = spawned {
        log::warn!("Could not start the signal handler: {e}");
    }
}

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

    let instance = match instance::acquire() {
        instance::Instance::Primary(primary) => Arc::new(Mutex::new(Some(primary))),
        instance::Instance::Secondary => {
            println!("FXSound is already running; bringing its window to the front.");
            return;
        }
    };
    let instance_for_setup = Arc::clone(&instance);

    let app = tauri::Builder::default()
        // Logs go to stdout and to the app's log directory
        // (~/.local/share/com.fxsound.linux/logs), so bug reports can include them.
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .max_file_size(512 * 1024)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .setup(move |app| {
            let config_file = app
                .path()
                .app_config_dir()
                .ok()
                .map(|dir| dir.join("settings.json"));
            let settings = Arc::new(SettingsStore::open(config_file));

            // Apply the saved state before any audio flows, so there is no
            // moment of wrong processing at launch.
            let engine = Arc::new(Mutex::new(AudioEngine::new()));
            let saved = settings.get();
            if let Some(saved) = &saved {
                let mut engine = engine.lock().unwrap_or_else(|e| e.into_inner());
                for (band, &gain) in saved.eq.iter().enumerate() {
                    engine.set_eq_band(band, gain);
                }
                for (name, value) in saved.effects.iter() {
                    engine.set_effect(name, value);
                }
                engine.set_power(saved.powered);
            }

            let handle = app.handle().clone();
            let store = Arc::clone(&settings);
            let audio = AudioService::start(
                Arc::clone(&engine),
                saved.and_then(|s| s.output_device),
                move |status| {
                    // A device picked in the system sound settings is followed
                    // and remembered just like one picked in FXSound.
                    if status.preferred.is_some() {
                        store.update(|s| s.output_device = status.preferred.clone());
                    }
                    let _ = handle.emit("audio-status", status);
                },
            );

            app.manage(AppState {
                engine,
                audio,
                settings,
            });

            // A second launch brings this window forward instead of starting
            // a competing copy.
            let handle = app.handle().clone();
            if let Some(primary) = instance_for_setup
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .as_mut()
            {
                primary.serve(move || {
                    if let Some(window) = handle.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                });
            }

            exit_on_signals(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_eq_band,
            set_effect,
            apply_preset_state,
            set_power,
            get_settings,
            get_audio_status,
            set_output_device,
            get_visualizer_data,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |handle, event| {
        if let RunEvent::Exit = event {
            if let Some(state) = handle.try_state::<AppState>() {
                state.audio.shutdown(Duration::from_secs(3));
                state.settings.flush();
            }
            instance.lock().unwrap_or_else(|e| e.into_inner()).take();
        }
    });
}
