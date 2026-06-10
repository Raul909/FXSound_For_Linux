## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-10 - IPC Memory Exhaustion DoS
**Vulnerability:** Unbounded insertion of untrusted IPC string keys into a backend HashMap via the `set_effect` command.
**Learning:** Inter-Process Communication (IPC) command parameters from the frontend must be treated as untrusted user input. Inserting them directly into dynamically sized collections like HashMaps without validation allows malicious payloads to cause memory exhaustion.
**Prevention:** Always validate and allow-list string parameters from IPC before using them as keys in dynamically sized collections.
