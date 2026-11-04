//! Picks how the WebKitGTK window is rendered: on the GPU, or with Mesa's
//! software rasteriser.
//!
//! Software rendering always starts, but it paints every frame on the CPU,
//! which kept a core busy just for the visualizer. The GPU path is cheaper but
//! has failed on some systems (VMs without 3D acceleration, NVIDIA's
//! proprietary driver, some Wayland compositors): WebKitGTK either aborts with
//! `EGL_BAD_PARAMETER` or the window stops responding after the first paint.
//!
//! So `Auto` uses the GPU only where it is known to work (a real Mesa render
//! node under X11), and every GPU launch is guarded: a marker file is written
//! before the window starts and removed once the UI reports it is up. If a GPU
//! launch dies before that, the next launch finds the marker, records the
//! failure and renders in software from then on, even when GPU was chosen
//! explicitly (otherwise the app could never open again to undo the choice).
//! Changing the rendering option in the settings clears the record, so the
//! GPU gets a fresh try.
//!
//! Everything here runs before Tauri and WebKitGTK start, because the
//! environment variables only take effect if they are set before the first
//! EGL display is created.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The user's rendering choice, saved in the settings file.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    #[default]
    Auto,
    Gpu,
    Software,
}

/// What this launch actually uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Renderer {
    Gpu,
    Software,
}

/// Why the renderer was chosen, shown in the settings page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    UserChoice,
    EnvironmentOverride,
    Detected,
    VirtualMachine,
    NoRenderNode,
    NvidiaDriver,
    Wayland,
    PreviousLaunchFailed,
}

/// Facts about the machine that the decision depends on.
#[derive(Clone, Copy, Debug, Default)]
pub struct System {
    pub env_override: bool,
    pub virtual_machine: bool,
    pub render_node: bool,
    pub nvidia_driver: bool,
    pub wayland: bool,
    pub previous_gpu_launch_failed: bool,
}

/// Pure decision, kept separate from the probing so it can be tested.
pub fn decide(mode: RenderMode, sys: System) -> (Renderer, Reason) {
    if sys.env_override {
        let software = std::env::var("LIBGL_ALWAYS_SOFTWARE").is_ok_and(|v| v != "0");
        let renderer = if software {
            Renderer::Software
        } else {
            Renderer::Gpu
        };
        return (renderer, Reason::EnvironmentOverride);
    }
    if sys.previous_gpu_launch_failed && mode != RenderMode::Software {
        return (Renderer::Software, Reason::PreviousLaunchFailed);
    }
    match mode {
        RenderMode::Gpu => (Renderer::Gpu, Reason::UserChoice),
        RenderMode::Software => (Renderer::Software, Reason::UserChoice),
        RenderMode::Auto => {
            let blocker = if sys.virtual_machine {
                Some(Reason::VirtualMachine)
            } else if !sys.render_node {
                Some(Reason::NoRenderNode)
            } else if sys.nvidia_driver {
                Some(Reason::NvidiaDriver)
            } else if sys.wayland {
                Some(Reason::Wayland)
            } else {
                None
            };
            match blocker {
                Some(reason) => (Renderer::Software, reason),
                None => (Renderer::Gpu, Reason::Detected),
            }
        }
    }
}

/// The renderer in use and why, for the settings page.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Choice {
    pub renderer: Renderer,
    pub reason: Reason,
}

/// Probe the machine, apply the decision to this process's environment and
/// arm the crash guard. Must run before Tauri starts.
pub fn apply(mode: RenderMode, config_dir: Option<&Path>) -> Choice {
    let marker = config_dir.map(|d| d.join(PENDING));
    let failed = config_dir.map(|d| d.join(FAILED));
    // A GPU launch that never reported in: remember it as a failure.
    if let (Some(marker), Some(failed)) = (&marker, &failed) {
        if marker.exists() {
            log::warn!("The last GPU launch did not start; rendering in software from now on");
            let _ = std::fs::rename(marker, failed);
        }
    }
    let sys = System {
        env_override: std::env::var_os("LIBGL_ALWAYS_SOFTWARE").is_some(),
        virtual_machine: is_virtual_machine(),
        render_node: has_render_node(),
        nvidia_driver: Path::new("/proc/driver/nvidia").exists(),
        wayland: is_wayland(),
        previous_gpu_launch_failed: failed.as_deref().is_some_and(Path::exists),
    };
    let (renderer, reason) = decide(mode, sys);
    log::info!("Rendering on the {renderer:?} ({reason:?})");

    match renderer {
        Renderer::Software => {
            set_default("LIBGL_ALWAYS_SOFTWARE", "1");
            set_default("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
            set_default("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
        Renderer::Gpu => {
            // Nothing to set: the DMABUF renderer is what lets WebKit hand
            // frames to the compositor without copying them through the CPU
            // (measured: idle window 24% of a core without it, 4% with it).
            // The drivers it misbehaved on are excluded above or caught by
            // the launch guard.
            // An environment override is the user's own experiment; only
            // our own choices are guarded.
            if reason != Reason::EnvironmentOverride {
                if let Some(marker) = &marker {
                    if let Some(dir) = marker.parent() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    let _ = std::fs::write(marker, b"");
                }
            }
        }
    }
    Choice { renderer, reason }
}

const PENDING: &str = "gpu-launch.pending";
const FAILED: &str = "gpu-failed";

/// The UI came up: this GPU launch worked.
pub fn confirm_started(config_dir: Option<&Path>) {
    if let Some(dir) = config_dir {
        let _ = std::fs::remove_file(dir.join(PENDING));
    }
}

/// The user changed the rendering option: forget a recorded GPU failure so
/// the new choice is tried as asked.
pub fn forget_failure(config_dir: Option<&Path>) {
    if let Some(dir) = config_dir {
        let _ = std::fs::remove_file(dir.join(FAILED));
    }
}

/// `~/.config/com.fxsound.linux`, worked out without Tauri (it is not running
/// yet). Must match `app_config_dir()` for the bundle identifier.
pub fn config_dir(identifier: &str) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join(identifier))
}

fn set_default(key: &str, value: &str) {
    if std::env::var_os(key).is_none() {
        std::env::set_var(key, value);
    }
}

fn is_virtual_machine() -> bool {
    // The CPU advertises the "hypervisor" flag inside every common hypervisor.
    std::fs::read_to_string("/proc/cpuinfo")
        .map(|info| {
            info.lines()
                .filter(|l| l.starts_with("flags"))
                .any(|l| l.split_whitespace().any(|f| f == "hypervisor"))
        })
        .unwrap_or(false)
}

fn has_render_node() -> bool {
    std::fs::read_dir("/dev/dri")
        .map(|dir| {
            dir.flatten()
                .any(|e| e.file_name().to_string_lossy().starts_with("renderD"))
        })
        .unwrap_or(false)
}

fn is_wayland() -> bool {
    std::env::var("XDG_SESSION_TYPE").is_ok_and(|t| t == "wayland")
        || std::env::var_os("WAYLAND_DISPLAY").is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_gpu() -> System {
        System {
            render_node: true,
            ..System::default()
        }
    }

    #[test]
    fn auto_uses_the_gpu_on_a_plain_mesa_x11_machine() {
        assert_eq!(
            decide(RenderMode::Auto, real_gpu()),
            (Renderer::Gpu, Reason::Detected)
        );
    }

    #[test]
    fn auto_falls_back_where_the_gpu_path_is_known_to_break() {
        let cases = [
            (
                System {
                    virtual_machine: true,
                    ..real_gpu()
                },
                Reason::VirtualMachine,
            ),
            (
                System {
                    render_node: false,
                    ..real_gpu()
                },
                Reason::NoRenderNode,
            ),
            (
                System {
                    nvidia_driver: true,
                    ..real_gpu()
                },
                Reason::NvidiaDriver,
            ),
            (
                System {
                    wayland: true,
                    ..real_gpu()
                },
                Reason::Wayland,
            ),
        ];
        for (sys, reason) in cases {
            assert_eq!(decide(RenderMode::Auto, sys), (Renderer::Software, reason));
        }
    }

    #[test]
    fn a_failed_gpu_launch_switches_to_software_even_when_gpu_was_chosen() {
        let sys = System {
            previous_gpu_launch_failed: true,
            ..real_gpu()
        };
        for mode in [RenderMode::Auto, RenderMode::Gpu] {
            assert_eq!(
                decide(mode, sys),
                (Renderer::Software, Reason::PreviousLaunchFailed)
            );
        }
        assert_eq!(
            decide(RenderMode::Software, sys),
            (Renderer::Software, Reason::UserChoice)
        );
    }

    #[test]
    fn a_gpu_launch_that_never_reports_in_is_remembered() {
        let dir = std::env::temp_dir().join(format!("fxsound-gfx-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(PENDING), b"").unwrap();
        // The crash record survives a software launch...
        apply(RenderMode::Software, Some(&dir));
        assert!(dir.join(FAILED).exists() && !dir.join(PENDING).exists());
        // ...until the user changes the option.
        forget_failure(Some(&dir));
        assert!(!dir.join(FAILED).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_explicit_choice_wins_over_detection() {
        let vm = System {
            virtual_machine: true,
            ..System::default()
        };
        assert_eq!(
            decide(RenderMode::Gpu, vm),
            (Renderer::Gpu, Reason::UserChoice)
        );
        assert_eq!(
            decide(RenderMode::Software, real_gpu()),
            (Renderer::Software, Reason::UserChoice)
        );
    }

    #[test]
    fn render_mode_parses_lowercase_names() {
        let m: RenderMode = serde_json::from_str("\"software\"").unwrap();
        assert_eq!(m, RenderMode::Software);
        assert!(serde_json::from_str::<RenderMode>("\"bogus\"").is_err());
    }
}
