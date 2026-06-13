## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2026-06-13 - Prevent IPC Memory Exhaustion (DoS)
**Vulnerability:** The `set_effect` Tauri command accepts an arbitrary string from the frontend IPC and inserts it directly into a Rust HashMap without validation. A malicious or compromised frontend could repeatedly send unique string keys, causing unbounded memory growth and eventually crashing the application (Denial of Service).
**Learning:** Treat all frontend IPC inputs as untrusted data in Tauri applications. Dynamically sizing collections like HashMaps must never accept unbounded strings directly from user input.
**Prevention:** Implement strict allow-lists or enums for IPC string parameters before using them as keys in backend data structures.
