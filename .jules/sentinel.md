## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-16 - Fix IPC Memory Exhaustion DoS
**Vulnerability:** The application accepted untrusted arbitrary strings from the frontend via Tauri IPC and inserted them directly into a dynamically sized HashMap (`self.effects`). An attacker could send an infinite number of unique strings, causing unbounded memory growth and eventually crashing the application (Denial of Service).
**Learning:** Always treat IPC parameters from the frontend as untrusted user input, especially when they are used as keys in collections.
**Prevention:** Implement strict allow-listing for string parameters before inserting them into HashMaps or other dynamically sized data structures.
