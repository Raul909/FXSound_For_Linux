//! Keeps FXSound to one running copy.
//!
//! Two copies would both try to own the FXSound audio device, each treating
//! the other's as a leftover to clean up. The first copy listens on a Unix
//! socket; a second launch finds it, asks it to show its window, and exits.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::time::Duration;

pub enum Instance {
    /// This is the only copy; call [`Primary::serve`] once the UI exists.
    Primary(Primary),
    /// Another copy is running and has been asked to show itself.
    Secondary,
}

pub struct Primary {
    listener: Option<UnixListener>,
    path: PathBuf,
    /// Whether the socket file is ours to remove on exit.
    owns_socket: bool,
}

fn socket_path() -> PathBuf {
    // XDG_RUNTIME_DIR is per-user (and per-snap under confinement).
    let dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|d| d.is_dir())
        .unwrap_or_else(std::env::temp_dir);
    // SAFETY: getuid() has no preconditions and cannot fail.
    let uid = unsafe { libc::getuid() };
    dir.join(format!("fxsound-linux-{uid}.sock"))
}

pub fn acquire() -> Instance {
    let path = socket_path();
    for _ in 0..2 {
        match UnixListener::bind(&path) {
            Ok(listener) => {
                return Instance::Primary(Primary {
                    listener: Some(listener),
                    path,
                    owns_socket: true,
                })
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                if let Ok(mut stream) = UnixStream::connect(&path) {
                    let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));
                    let _ = stream.write_all(b"show\n");
                    return Instance::Secondary;
                }
                // Nobody is listening: a copy that crashed left its socket.
                let _ = std::fs::remove_file(&path);
            }
            Err(e) => {
                // Can't coordinate (read-only runtime dir?) — run anyway
                // rather than refuse to start.
                log::warn!("Single-instance check unavailable: {e}");
                break;
            }
        }
    }
    Instance::Primary(Primary {
        listener: None,
        path,
        owns_socket: false,
    })
}

impl Primary {
    /// Call `on_show` whenever another launch asks this copy to come forward.
    pub fn serve(&mut self, on_show: impl Fn() + Send + 'static) {
        let Some(listener) = self.listener.take() else {
            return;
        };
        let spawned = std::thread::Builder::new()
            .name("fxsound-instance".into())
            .spawn(move || {
                for stream in listener.incoming().flatten() {
                    let mut stream = stream;
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
                    let mut buf = [0u8; 16];
                    if matches!(stream.read(&mut buf), Ok(n) if buf[..n].starts_with(b"show")) {
                        on_show();
                    }
                }
            });
        if let Err(e) = spawned {
            log::warn!("Could not start the single-instance listener: {e}");
        }
    }
}

impl Drop for Primary {
    fn drop(&mut self) {
        if self.owns_socket {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
