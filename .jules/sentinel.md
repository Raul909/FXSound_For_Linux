## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-23 - Prevent IPC Memory Exhaustion (DoS)
**Vulnerability:** Unbounded insertion of untrusted string parameters into a dynamically sized HashMap via Tauri IPC commands. An attacker can send arbitrary effect names, causing memory exhaustion and Denial of Service.
**Learning:** Always treat IPC arguments from the frontend as untrusted user input. Inserting them directly into data structures like HashMaps without validation opens the application up to resource exhaustion attacks.
**Prevention:** Implement strict allow-listing for all string parameters passed over IPC before using them as keys in dynamically sized collections.
