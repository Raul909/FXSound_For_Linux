## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-28 - Prevent IPC Memory Exhaustion DoS
**Vulnerability:** The `set_effect` Tauri command accepted arbitrary effect strings from the frontend IPC boundary and inserted them directly into a dynamically sized `HashMap`, creating a memory exhaustion (DoS) vulnerability if flooded with random keys.
**Learning:** All untrusted input from the frontend via IPC commands must be treated as hostile and strictly validated. Dynamically sized collections should never be populated with unvalidated unbounded keys.
**Prevention:** Implement strict allow-listing for string parameters before inserting them into maps or collections.
