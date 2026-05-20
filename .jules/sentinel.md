## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-20 - Fix Unbounded Map Growth Denial of Service (DoS)
**Vulnerability:** The `set_effect` Tauri IPC command previously accepted any string key and inserted it directly into a dynamically sized `HashMap`. A malicious frontend or compromised payload could send millions of unique keys, exhausting system memory.
**Learning:** In Tauri, IPC command arguments must be treated as untrusted user input, even if the frontend is considered "first-party." Unbounded dynamic allocations based on user strings are a DoS vector.
**Prevention:** Always validate and allow-list string parameters before using them as keys in dynamically sized collections like HashMaps.
