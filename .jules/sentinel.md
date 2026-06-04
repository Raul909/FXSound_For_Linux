## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-06-04 - Prevent IPC Memory Exhaustion in Tauri Commands
**Vulnerability:** The `set_effect` Tauri command accepted arbitrary strings from the frontend and blindly inserted them into a backend HashMap, enabling a Denial of Service (DoS) attack via memory exhaustion.
**Learning:** Inter-Process Communication (IPC) command parameters from the frontend must be treated as untrusted user input. Dynamically sized collections (like HashMaps) can easily become unbounded without strict allow-lists.
**Prevention:** Always validate and allow-list string parameters before processing or inserting them into memory structures. Check frontend constants to ensure all valid UI keys are included in the backend allow-list.
