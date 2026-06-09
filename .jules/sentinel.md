## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-09 - Prevent IPC HashMap Memory Exhaustion (DoS)
**Vulnerability:** The Tauri IPC command set_effect accepted arbitrary effect string keys from the frontend and directly inserted them into a backend HashMap.
**Learning:** In Tauri apps, IPC commands must treat frontend arguments as untrusted user input. Inserting unbounded, unvalidated strings into a HashMap allows an attacker to exhaust system memory.
**Prevention:** Always validate and allow-list string parameters before inserting them into dynamically sized collections in backend command handlers.
