//! FXSound Tauri application entry point and command handlers.
//!
//! Wires the audio engine, the system audio routing (`pulse.rs`) and the saved
//! settings to the React frontend, and makes sure the system's own audio
//! routing comes back however the app exits.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, RunEvent, State, WindowEvent, Wry};
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_dialog::DialogExt as _;

mod audio;
mod graphics;
mod i18n;
mod instance;
mod pulse;
mod settings;

use audio::{AudioEngine, EFFECT_NAMES};
use pulse::{AudioService, RoutingStatus};
use settings::{Settings, SettingsStore, UiSettings};

/// Bundle identifier; also names the config directory.
const IDENTIFIER: &str = "com.fxsound.linux";
/// Passed by the session autostart entry, so a login start can stay in the tray.
const AUTOSTART_ARG: &str = "--autostart";

/// Shared application state behind the Tauri commands.
struct AppState {
    engine: Arc<Mutex<AudioEngine>>,
    audio: AudioService,
    settings: Arc<SettingsStore>,
    /// How this launch renders, decided before the window existed.
    graphics: graphics::Choice,
    config_dir: Option<PathBuf>,
    /// `None` when the desktop has no tray to put the icon in.
    tray: Option<TrayHandles>,
}

/// Tray menu items that mirror state changed elsewhere.
struct TrayHandles {
    show: MenuItem<Wry>,
    hide: MenuItem<Wry>,
    power: CheckMenuItem<Wry>,
    quit: MenuItem<Wry>,
}

impl TrayHandles {
    fn relabel(&self, lang: &str) {
        let t = i18n::texts(lang);
        let _ = self.show.set_text(t.show);
        let _ = self.hide.set_text(t.hide);
        let _ = self.power.set_text(t.effects_on);
        let _ = self.quit.set_text(t.quit);
    }
}

impl AppState {
    /// The interface language in effect ("auto" resolved from the system).
    fn language(&self) -> &'static str {
        i18n::resolve(&self.settings.get().unwrap_or_default().ui.language)
    }
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
    apply_power(&state, enabled);
    Ok(())
}

/// One place that changes power, so the window and the tray never disagree.
fn apply_power(state: &AppState, enabled: bool) {
    state.engine().set_power(enabled);
    state.settings.update(|s| s.powered = enabled);
    if let Some(tray) = &state.tray {
        let _ = tray.power.set_checked(enabled);
    }
}

/// Everything the settings page shows.
#[derive(serde::Serialize)]
struct UiInfo {
    ui: UiSettings,
    graphics: graphics::Choice,
    tray_available: bool,
    autostart: bool,
    /// Effective language code, for the page to translate itself.
    language: &'static str,
}

#[tauri::command]
fn get_ui_info(app: AppHandle, state: State<AppState>) -> UiInfo {
    UiInfo {
        ui: state.settings.get().unwrap_or_default().ui,
        graphics: state.graphics,
        tray_available: state.tray.is_some(),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        language: state.language(),
    }
}

/// Save the window/rendering options. Rendering changes apply next launch.
#[tauri::command]
fn set_ui_settings(state: State<AppState>, mut ui: UiSettings) -> Result<&'static str, String> {
    if ui.language != "auto" && !i18n::SUPPORTED.contains(&ui.language.as_str()) {
        ui.language = "auto".into();
    }
    let before = state.settings.get().unwrap_or_default().ui;
    if ui.render_mode != before.render_mode {
        graphics::forget_failure(state.config_dir.as_deref());
    }
    state.settings.update(|s| s.ui = ui);
    let language = state.language();
    if let Some(tray) = &state.tray {
        tray.relabel(language);
    }
    Ok(language)
}

/// Start FXSound with the desktop session (an XDG autostart entry).
#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<bool, String> {
    let launcher = app.autolaunch();
    let result = if enabled {
        launcher.enable()
    } else {
        launcher.disable()
    };
    result.map_err(|e| format!("Could not change autostart: {e}"))?;
    launcher.is_enabled().map_err(|e| e.to_string())
}

/// The window loaded: an automatic GPU choice is proven to work here.
#[tauri::command]
fn ui_ready(state: State<AppState>) {
    graphics::confirm_started(state.config_dir.as_deref());
}

/// Ask where to save, then write the configuration there. `Ok(false)` means
/// the user cancelled. The path never comes from the web page.
#[tauri::command]
async fn export_settings(app: AppHandle, state: State<'_, AppState>) -> Result<bool, String> {
    let t = i18n::texts(state.language());
    let Some(path) = app
        .dialog()
        .file()
        .set_title(t.export_title)
        .add_filter(t.file_filter, &["json"])
        .set_file_name("fxsound-config.json")
        .blocking_save_file()
    else {
        return Ok(false);
    };
    let mut path = path.into_path().map_err(|e| e.to_string())?;
    if path.extension().is_none() {
        path.set_extension("json");
    }
    let text = settings::export_json(&state.settings.get().unwrap_or_default());
    std::fs::write(&path, text).map_err(|e| format!("Could not write {}: {e}", path.display()))?;
    log::info!("Exported configuration to {}", path.display());
    Ok(true)
}

/// Ask for a file, validate it and make it the current configuration.
/// Returns the new settings, or `None` if the user cancelled. The output
/// device in use is kept: it belongs to this machine, not to the file.
#[tauri::command]
async fn import_settings(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<Settings>, String> {
    let t = i18n::texts(state.language());
    let Some(path) = app
        .dialog()
        .file()
        .set_title(t.import_title)
        .add_filter(t.file_filter, &["json"])
        .blocking_pick_file()
    else {
        return Ok(None);
    };
    let path = path.into_path().map_err(|e| e.to_string())?;
    let size = std::fs::metadata(&path).map_err(|e| e.to_string())?.len();
    if size > settings::MAX_IMPORT_BYTES {
        return Err(settings::ERR_NOT_CONFIG.into());
    }
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("Could not read {}: {e}", path.display()))?;
    let mut imported = settings::import_json(&text)?;
    imported.output_device = state.settings.get().and_then(|s| s.output_device);

    {
        let mut engine = state.engine();
        for (band, &gain) in imported.eq.iter().enumerate() {
            engine.set_eq_band(band, gain);
        }
        for (name, value) in imported.effects.iter() {
            engine.set_effect(name, value);
        }
    }
    apply_power(&state, imported.powered);
    let stored = imported.clone();
    state.settings.update(move |s| *s = stored);
    if let Some(tray) = &state.tray {
        tray.relabel(state.language());
    }
    log::info!("Imported configuration from {}", path.display());
    Ok(Some(imported))
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

/// Bring the window back from the tray, the taskbar or behind other windows.
fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
    let _ = app.emit("window-visibility", true);
}

/// Hide to the tray. The page is told, so the visualizer stops working.
fn hide_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    let _ = app.emit("window-visibility", false);
}

fn build_tray(app: &AppHandle, powered: bool, lang: &str) -> tauri::Result<TrayHandles> {
    let t = i18n::texts(lang);
    let show = MenuItem::with_id(app, "show", t.show, true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", t.hide, true, None::<&str>)?;
    let power = CheckMenuItem::with_id(app, "power", t.effects_on, true, powered, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", t.quit, true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&show, &hide, &separator, &power, &separator, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip("FXSound")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show_window(app),
            "hide" => hide_window(app),
            "power" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let enabled = !state.settings.get().map(|s| s.powered).unwrap_or(true);
                    apply_power(&state, enabled);
                    let _ = app.emit("power-changed", enabled);
                }
            }
            "quit" => app.exit(0),
            _ => {}
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(TrayHandles {
        show,
        hide,
        power,
        quit,
    })
}

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
    let instance = match instance::acquire() {
        instance::Instance::Primary(primary) => Arc::new(Mutex::new(Some(primary))),
        instance::Instance::Secondary => {
            println!("FXSound is already running; bringing its window to the front.");
            return;
        }
    };
    let instance_for_setup = Arc::clone(&instance);

    // Settings are read before Tauri starts: the rendering choice has to be
    // in the environment before WebKitGTK creates its first EGL display.
    let config_dir = graphics::config_dir(IDENTIFIER);
    let settings = Arc::new(SettingsStore::open(
        config_dir.as_ref().map(|dir| dir.join("settings.json")),
    ));
    let saved_ui = settings.get().unwrap_or_default().ui;
    let graphics = graphics::apply(saved_ui.render_mode, config_dir.as_deref());
    let autostarted = std::env::args().any(|a| a == AUTOSTART_ARG);

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![AUTOSTART_ARG]),
        ))
        .setup(move |app| {
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

            // Decided before logging existed; record it now for bug reports.
            log::info!(
                "Window rendering: {:?} ({:?})",
                graphics.renderer,
                graphics.reason
            );
            let powered = settings.get().map(|s| s.powered).unwrap_or(true);
            let language = i18n::resolve(&saved_ui.language);
            let tray = match build_tray(app.handle(), powered, language) {
                Ok(tray) => Some(tray),
                Err(e) => {
                    // Without a tray, hiding the window would strand the app.
                    log::warn!("No system tray available ({e}); closing the window will quit");
                    None
                }
            };
            let start_hidden = autostarted && saved_ui.start_hidden && tray.is_some();

            app.manage(AppState {
                engine,
                audio,
                settings,
                graphics,
                config_dir: config_dir.clone(),
                tray,
            });

            if !start_hidden {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                }
            }

            // A second launch brings this window forward instead of starting
            // a competing copy.
            let handle = app.handle().clone();
            if let Some(primary) = instance_for_setup
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .as_mut()
            {
                primary.serve(move || show_window(&handle));
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
            get_ui_info,
            set_ui_settings,
            set_autostart,
            ui_ready,
            export_settings,
            import_settings,
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                let Some(state) = app.try_state::<AppState>() else {
                    return;
                };
                let to_tray = state.tray.is_some()
                    && state.settings.get().unwrap_or_default().ui.close_to_tray;
                if to_tray {
                    api.prevent_close();
                    hide_window(app);
                }
            }
        })
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
