## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-06-11 - Prevent Unbounded HashMap Growth via IPC (Memory Exhaustion DoS)
**Vulnerability:** Tauri IPC commands accepted untrusted, arbitrary strings for effect names from the frontend and inserted them directly into a backend `HashMap` (`self.effects`). A malicious or compromised frontend could repeatedly send random strings to exhaust backend memory, causing a Denial of Service (DoS).
**Learning:** In Tauri applications, Inter-Process Communication (IPC) parameters from the frontend must always be treated as untrusted user input, especially when used as keys for dynamically sized collections in Rust.
**Prevention:** Always validate and strictly allow-list string inputs from the frontend before inserting them into `HashMap` or `Vec` structures to prevent memory exhaustion attacks.
