## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-06-14 - Fix IPC memory exhaustion in effect processing
**Vulnerability:** Untrusted IPC input for effect names was inserted directly into a dynamically sized HashMap without validation, leading to potential memory exhaustion and Denial of Service.
**Learning:** In Tauri applications, treat Inter-Process Communication (IPC) command parameters from the frontend as untrusted user input. Always validate and allow-list string parameters before inserting them into dynamically sized collections to prevent memory exhaustion attacks. Make sure to check the frontend constants so that valid effects aren't disabled.
**Prevention:** Always validate and allow-list string parameters before inserting them into dynamically sized collections like HashMaps.
