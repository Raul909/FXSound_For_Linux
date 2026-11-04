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
    UnsupportedDriver,
    NvidiaDriver,
    Wayland,
    PreviousLaunchFailed,
    /// The launch guard could not be armed (config directory not writable),
    /// so the GPU path is not taken unprotected.
    GuardUnavailable,
}

/// Facts about the machine that the decision depends on.
#[derive(Clone, Copy, Debug, Default)]
pub struct System {
    pub env_override: bool,
    pub virtual_machine: bool,
    pub render_node: bool,
    /// A render node is driven by a kernel driver with a Mesa userspace.
    pub mesa_driver: bool,
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
            } else if !sys.mesa_driver {
                Some(Reason::UnsupportedDriver)
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
    let (mut renderer, mut reason) = decide(mode, probe(config_dir));

    (renderer, reason) = guarded((renderer, reason), marker.as_deref());
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
        }
    }
    Choice { renderer, reason }
}

/// Facts about this machine right now, without changing anything.
pub fn probe(config_dir: Option<&Path>) -> System {
    let (render_node, mesa_driver) = render_nodes();
    System {
        env_override: std::env::var_os("LIBGL_ALWAYS_SOFTWARE").is_some(),
        virtual_machine: is_virtual_machine(),
        render_node,
        mesa_driver,
        nvidia_driver: Path::new("/proc/driver/nvidia").exists(),
        wayland: is_wayland(),
        previous_gpu_launch_failed: config_dir.is_some_and(|d| d.join(FAILED).exists()),
    }
}

/// What the next launch would render with for `mode`, so the settings page
/// can say whether a restart changes anything.
pub fn next_renderer(mode: RenderMode, config_dir: Option<&Path>) -> Renderer {
    decide(mode, probe(config_dir)).0
}

/// Our own GPU choices only run guarded; if the marker cannot be written,
/// fail closed to software. An environment override is the user's own
/// experiment and is left alone.
fn guarded(choice: (Renderer, Reason), marker: Option<&Path>) -> (Renderer, Reason) {
    if choice.0 != Renderer::Gpu || choice.1 == Reason::EnvironmentOverride {
        return choice;
    }
    match arm_guard(marker) {
        Ok(()) => choice,
        Err(e) => {
            log::warn!("Cannot arm the GPU launch guard ({e}); rendering in software");
            (Renderer::Software, Reason::GuardUnavailable)
        }
    }
}

fn arm_guard(marker: Option<&Path>) -> std::io::Result<()> {
    let marker = marker.ok_or_else(|| std::io::Error::other("no config directory"))?;
    if let Some(dir) = marker.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(marker, b"")
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

/// Kernel DRM drivers whose GPUs are driven by Mesa in userspace.
const MESA_DRIVERS: [&str; 5] = ["amdgpu", "radeon", "i915", "xe", "nouveau"];

/// (any render node exists, one of them is driven by a Mesa driver)
fn render_nodes() -> (bool, bool) {
    let Ok(dir) = std::fs::read_dir("/sys/class/drm") else {
        return (false, false);
    };
    let mut any = false;
    let mut mesa = false;
    for entry in dir.flatten() {
        if !entry.file_name().to_string_lossy().starts_with("renderD") {
            continue;
        }
        any = true;
        let driver = std::fs::read_link(entry.path().join("device/driver")).ok();
        let name = driver
            .as_deref()
            .and_then(Path::file_name)
            .map(|n| n.to_string_lossy().into_owned());
        mesa |= name.is_some_and(|n| MESA_DRIVERS.contains(&n.as_str()));
    }
    (any, mesa)
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
            mesa_driver: true,
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
                    mesa_driver: false,
                    ..real_gpu()
                },
                Reason::UnsupportedDriver,
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
    fn a_gpu_choice_falls_back_to_software_when_the_guard_cannot_be_armed() {
        let dir = std::env::temp_dir().join(format!("fxsound-gfx-ro-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // A regular file where the config directory should be: nothing can
        // be created inside it.
        let blocker = dir.join("not-a-dir");
        std::fs::write(&blocker, b"").unwrap();
        let gpu = (Renderer::Gpu, Reason::Detected);
        let software = (Renderer::Software, Reason::GuardUnavailable);
        assert_eq!(guarded(gpu, Some(&blocker.join(PENDING))), software);
        assert_eq!(guarded(gpu, None), software);
        // Writable: the GPU stays, with the marker in place.
        let ok = dir.join("cfg").join(PENDING);
        assert_eq!(guarded(gpu, Some(&ok)), gpu);
        assert!(ok.exists());
        // Environment choices are never touched.
        let env = (Renderer::Gpu, Reason::EnvironmentOverride);
        assert_eq!(guarded(env, None), env);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn render_mode_parses_lowercase_names() {
        let m: RenderMode = serde_json::from_str("\"software\"").unwrap();
        assert_eq!(m, RenderMode::Software);
        assert!(serde_json::from_str::<RenderMode>("\"bogus\"").is_err());
    }
}
