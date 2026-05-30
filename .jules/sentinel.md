## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-30 - Fix IPC Memory Exhaustion DoS in set_effect
**Vulnerability:** The `set_effect` Tauri command accepted untrusted string parameters from the frontend and directly inserted them into an unbounded `HashMap` without validation, creating a memory exhaustion (DoS) vector.
**Learning:** In Tauri applications, Inter-Process Communication (IPC) command parameters must be treated as untrusted user input. Dynamically sized collections like `HashMap` can be exploited if keys are not strictly constrained.
**Prevention:** Always validate and allow-list string parameters from IPC before inserting them into state maps or dynamically sized collections.
