## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-05-21 - Prevent memory exhaustion DoS in Tauri IPC

**Vulnerability:** Untrusted IPC string parameters were directly inserted into a dynamically sized `HashMap` (`self.effects`) without validation, allowing a malicious frontend payload to cause unbounded memory growth (DoS).
**Learning:** In Tauri apps, IPC commands from the frontend must be treated as untrusted user input, especially when inserting into unbounded collections.
**Prevention:** Always use an allow-list to validate string parameters against known acceptable values before insertion into backend state collections.
