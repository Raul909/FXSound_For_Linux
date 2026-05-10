## 2026-05-10 - Fix unhandled Mutex poisoning DoS
**Vulnerability:** Unsafe Mutex lock `.unwrap()` calls in `src-tauri/src/audio.rs` could lead to a permanent Denial of Service if the locking thread panicked, poisoning the Mutex.
**Learning:** In Rust, if a thread panics while holding a `Mutex`, the Mutex becomes poisoned. Subsequent calls to `.lock()` will return an `Err`. Calling `.unwrap()` on this `Err` propagates the panic, permanently blocking other threads from accessing the state and causing a DoS.
**Prevention:** Always handle Mutex poisoning gracefully by recovering the lock using `.unwrap_or_else(|e| e.into_inner())` or properly bubbling up the error to prevent application crashes.
