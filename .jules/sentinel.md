## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-19 - Fix Memory Exhaustion and Log Injection in set_effect
**Vulnerability:** The `set_effect` Tauri command blindly accepted arbitrary string keys from the frontend and inserted them into a `HashMap`, creating a memory exhaustion (DoS) vulnerability. The untrusted input was also logged directly, enabling log injection.
**Learning:** All input crossing the IPC boundary from the frontend to the backend must be strictly validated. HashMaps storing arbitrary keys provided by the frontend can grow unboundedly, leading to Out-Of-Memory (OOM) crashes.
**Prevention:** Implement strict allow-lists for valid keys and use `.escape_default()` or similar mechanisms when logging untrusted input.
