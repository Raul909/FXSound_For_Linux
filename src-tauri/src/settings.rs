//! User settings, remembered between launches.
//!
//! Every launch used to start over on the Music preset with power on and the
//! system's output device, whatever the user had set up before. The backend
//! now keeps the EQ, effects, power state and chosen output device in
//! `settings.json` under the app's config directory and applies them before
//! any audio flows.

use crate::graphics::RenderMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Writes are coalesced: a slider drag produces dozens of changes a second,
/// but the file is rewritten at most this often.
const SAVE_DELAY: Duration = Duration::from_millis(400);

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Effects {
    pub fidelity: f32,
    pub ambiance: f32,
    pub dynamic: f32,
    pub surround: f32,
    pub bass: f32,
}

impl Effects {
    pub fn set(&mut self, name: &str, value: f32) {
        match name {
            "fidelity" => self.fidelity = value,
            "ambiance" => self.ambiance = value,
            "dynamic" => self.dynamic = value,
            "surround" => self.surround = value,
            "bass" => self.bass = value,
            _ => {}
        }
    }

    pub fn iter(&self) -> [(&'static str, f32); 5] {
        [
            ("fidelity", self.fidelity),
            ("ambiance", self.ambiance),
            ("dynamic", self.dynamic),
            ("surround", self.surround),
            ("bass", self.bass),
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub powered: bool,
    /// Preset name shown in the UI, or "Custom".
    pub preset: String,
    pub eq: [f32; 10],
    pub effects: Effects,
    /// Output device chosen by the user; `None` lets FXSound decide.
    pub output_device: Option<String>,
    /// Window and rendering behaviour (not audio).
    pub ui: UiSettings,
}

/// How the app window behaves. Files saved before these existed load with
/// the defaults below.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UiSettings {
    /// Draw the spectrum. Off stops all visualizer work, not only the drawing.
    pub visualizer: bool,
    /// Closing the window keeps FXSound running in the tray.
    pub close_to_tray: bool,
    /// When started with the session, stay in the tray instead of opening.
    pub start_hidden: bool,
    /// GPU or software rendering; applied at the next launch.
    pub render_mode: RenderMode,
    /// Interface language: "auto" (follow the system) or a code from
    /// `i18n::SUPPORTED`.
    pub language: String,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            visualizer: true,
            close_to_tray: true,
            start_hidden: true,
            render_mode: RenderMode::Auto,
            language: "auto".into(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            powered: true,
            preset: "Custom".into(),
            eq: [0.0; 10],
            effects: Effects::default(),
            output_device: None,
            ui: UiSettings::default(),
        }
    }
}

impl Settings {
    /// Clamp values that may have been edited by hand to what the engine
    /// accepts, and drop anything that is not a number.
    pub(crate) fn sanitized(mut self) -> Self {
        for gain in self.eq.iter_mut() {
            *gain = if gain.is_finite() {
                gain.clamp(-12.0, 12.0)
            } else {
                0.0
            };
        }
        for (name, value) in self.effects.iter() {
            let value = if value.is_finite() {
                value.clamp(0.0, 100.0)
            } else {
                0.0
            };
            self.effects.set(name, value);
        }
        self.output_device = self.output_device.filter(|d| !d.is_empty());
        if !crate::i18n::SUPPORTED.contains(&self.ui.language.as_str()) {
            self.ui.language = "auto".into();
        }
        self
    }
}

/// Marks an exported file so an unrelated JSON file is rejected on import.
const EXPORT_FORMAT: &str = "fxsound-linux-config";
const EXPORT_VERSION: u32 = 1;
/// Error codes the page translates (see `src/i18n.js`).
pub const ERR_NOT_CONFIG: &str = "error.not-config";
pub const ERR_NEWER: &str = "error.newer";
/// A settings file is well under 2 KB; anything far bigger is not one.
pub const MAX_IMPORT_BYTES: u64 = 64 * 1024;

#[derive(Serialize, Deserialize)]
struct ExportFile {
    format: String,
    version: u32,
    settings: Settings,
}

/// The settings as a shareable file. The output device is left out: device
/// names belong to one machine and would not match on another.
pub fn export_json(settings: &Settings) -> String {
    let file = ExportFile {
        format: EXPORT_FORMAT.into(),
        version: EXPORT_VERSION,
        settings: Settings {
            output_device: None,
            ..settings.clone()
        },
    };
    serde_json::to_string_pretty(&file).expect("settings always serialize")
}

/// Read a file written by [`export_json`]. Values are clamped like a hand-edited
/// settings file; the output device is never taken from the file.
pub fn import_json(text: &str) -> Result<Settings, String> {
    let file: ExportFile = serde_json::from_str(text).map_err(|_| ERR_NOT_CONFIG.to_string())?;
    if file.format != EXPORT_FORMAT {
        return Err(ERR_NOT_CONFIG.into());
    }
    if file.version > EXPORT_VERSION {
        return Err(ERR_NEWER.into());
    }
    Ok(Settings {
        output_device: None,
        ..file.settings.sanitized()
    })
}

pub struct SettingsStore {
    path: Option<PathBuf>,
    /// `None` until the first launch has saved anything.
    current: Arc<Mutex<Option<Settings>>>,
    save: Mutex<Option<Sender<()>>>,
}

impl SettingsStore {
    /// Load settings from `path` (if any) and start the background writer.
    pub fn open(path: Option<PathBuf>) -> Self {
        let loaded = path
            .as_deref()
            .and_then(|p| match std::fs::read_to_string(p) {
                Ok(text) => match serde_json::from_str::<Settings>(&text) {
                    Ok(settings) => Some(settings.sanitized()),
                    Err(e) => {
                        log::warn!("Ignoring unreadable settings file {}: {e}", p.display());
                        None
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => {
                    log::warn!("Could not read settings file {}: {e}", p.display());
                    None
                }
            });

        let current = Arc::new(Mutex::new(loaded));
        let save = path.clone().map(|path| {
            let (tx, rx) = mpsc::channel::<()>();
            let current = Arc::clone(&current);
            std::thread::Builder::new()
                .name("fxsound-settings".into())
                .spawn(move || {
                    while rx.recv().is_ok() {
                        // Let a burst of changes settle, then write once.
                        loop {
                            match rx.recv_timeout(SAVE_DELAY) {
                                Ok(()) => continue,
                                Err(RecvTimeoutError::Timeout) => break,
                                Err(RecvTimeoutError::Disconnected) => break,
                            }
                        }
                        write(&path, &current);
                    }
                })
                .expect("failed to spawn the settings writer");
            tx
        });

        Self {
            path,
            current,
            save: Mutex::new(save),
        }
    }

    pub fn get(&self) -> Option<Settings> {
        self.current
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Change the settings and schedule a save.
    pub fn update(&self, change: impl FnOnce(&mut Settings)) {
        {
            let mut current = self.current.lock().unwrap_or_else(|e| e.into_inner());
            let mut next = current.clone().unwrap_or_default();
            change(&mut next);
            if current.as_ref() == Some(&next) {
                return;
            }
            *current = Some(next);
        }
        if let Some(save) = self.save.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
            let _ = save.send(());
        }
    }

    /// Write immediately and stop the background writer. Call on exit.
    pub fn flush(&self) {
        self.save.lock().unwrap_or_else(|e| e.into_inner()).take();
        if let Some(path) = &self.path {
            write(path, &self.current);
        }
    }
}

/// Atomically replace the settings file, so a crash mid-write can never
/// leave it truncated.
fn write(path: &PathBuf, current: &Mutex<Option<Settings>>) {
    let Some(settings) = current.lock().unwrap_or_else(|e| e.into_inner()).clone() else {
        return;
    };
    let result = (|| -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let text = serde_json::to_string_pretty(&settings).map_err(std::io::Error::other)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, text)?;
        std::fs::rename(&tmp, path)
    })();
    if let Err(e) = result {
        log::warn!("Could not save settings to {}: {e}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("fxsound-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir.join("settings.json")
    }

    #[test]
    fn round_trips_through_disk() {
        let path = temp_path("roundtrip");
        let store = SettingsStore::open(Some(path.clone()));
        assert_eq!(store.get(), None, "first launch has no settings");
        store.update(|s| {
            s.preset = "Bass Boost".into();
            s.eq[0] = 8.0;
            s.effects.set("bass", 90.0);
            s.powered = false;
            s.output_device = Some("alsa_output.usb".into());
        });
        store.flush();

        let reopened = SettingsStore::open(Some(path.clone())).get().unwrap();
        assert_eq!(reopened.preset, "Bass Boost");
        assert_eq!(reopened.eq[0], 8.0);
        assert_eq!(reopened.effects.bass, 90.0);
        assert!(!reopened.powered);
        assert_eq!(reopened.output_device.as_deref(), Some("alsa_output.usb"));
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn hand_edited_values_are_clamped_and_unknown_keys_ignored() {
        let path = temp_path("sanitize");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            r#"{"eq":[99,-99,0,0,0,0,0,0,0,0],"effects":{"bass":250},"output_device":"","future_key":1}"#,
        )
        .unwrap();
        let s = SettingsStore::open(Some(path.clone())).get().unwrap();
        assert_eq!(s.eq[0], 12.0);
        assert_eq!(s.eq[1], -12.0);
        assert_eq!(s.effects.bass, 100.0);
        assert_eq!(s.output_device, None);
        assert!(s.powered, "missing keys take their defaults");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn corrupt_file_is_ignored() {
        let path = temp_path("corrupt");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "{not json").unwrap();
        assert_eq!(SettingsStore::open(Some(path.clone())).get(), None);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}

#[cfg(test)]
mod export_tests {
    use super::*;

    fn sample() -> Settings {
        let mut s = Settings {
            preset: "Bass Boost".into(),
            output_device: Some("alsa_output.usb-headset".into()),
            ..Settings::default()
        };
        s.eq[0] = 6.5;
        s.effects.bass = 80.0;
        s.ui.visualizer = false;
        s.ui.render_mode = RenderMode::Software;
        s.ui.language = "pt".into();
        s
    }

    #[test]
    fn export_then_import_keeps_everything_but_the_device() {
        let back = import_json(&export_json(&sample())).unwrap();
        assert_eq!(
            back,
            Settings {
                output_device: None,
                ..sample()
            }
        );
    }

    #[test]
    fn import_rejects_files_that_are_not_ours() {
        assert!(import_json("{\"eq\":[1,2,3]}").is_err());
        assert!(import_json("not json").is_err());
        let foreign = export_json(&sample()).replace(EXPORT_FORMAT, "something-else");
        assert!(import_json(&foreign).is_err());
    }

    #[test]
    fn import_refuses_a_newer_format() {
        let newer = export_json(&sample()).replace("\"version\": 1", "\"version\": 99");
        assert!(import_json(&newer).unwrap_err().contains("newer"));
    }

    #[test]
    fn import_clamps_out_of_range_values() {
        let wild = export_json(&sample()).replace("6.5", "400.0");
        assert_eq!(import_json(&wild).unwrap().eq[0], 12.0);
    }

    #[test]
    fn an_unknown_language_falls_back_to_automatic() {
        let file = export_json(&sample()).replace("\"pt\"", "\"klingon\"");
        assert_eq!(import_json(&file).unwrap().ui.language, "auto");
    }

    #[test]
    fn settings_saved_before_ui_options_load_with_defaults() {
        let old = r#"{"powered":true,"preset":"Music","eq":[0,0,0,0,0,0,0,0,0,0],
                      "effects":{"bass":10},"output_device":null}"#;
        let s: Settings = serde_json::from_str(old).unwrap();
        assert_eq!(s.ui, UiSettings::default());
        assert_eq!(s.effects.bass, 10.0);
    }
}
