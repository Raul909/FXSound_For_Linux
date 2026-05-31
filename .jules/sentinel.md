## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-05-31 - IPC Memory Exhaustion (DoS)
**Vulnerability:** Untrusted string parameters from the frontend via Tauri IPC commands were inserted directly into a dynamically sized HashMap (`self.effects`) without validation, allowing potential memory exhaustion crashes.
**Learning:** In Tauri applications, all frontend IPC inputs must be treated as untrusted and validated.
**Prevention:** Implement an allowlist to validate strings before inserting them into unbounded collections.
