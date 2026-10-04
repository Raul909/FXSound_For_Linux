//! User settings, remembered between launches.
//!
//! Every launch used to start over on the Music preset with power on and the
//! system's output device, whatever the user had set up before. The backend
//! now keeps the EQ, effects, power state and chosen output device in
//! `settings.json` under the app's config directory and applies them before
//! any audio flows.

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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            powered: true,
            preset: "Custom".into(),
            eq: [0.0; 10],
            effects: Effects::default(),
            output_device: None,
        }
    }
}

impl Settings {
    /// Clamp values that may have been edited by hand to what the engine
    /// accepts, and drop anything that is not a number.
    fn sanitized(mut self) -> Self {
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
        self
    }
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
