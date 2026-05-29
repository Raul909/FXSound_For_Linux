## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-29 - Prevent IPC Memory Exhaustion (DoS)
**Vulnerability:** The Tauri command `set_effect` accepts an arbitrary `String` from the frontend and inserts it as a key into a dynamically sized `HashMap` (`self.effects`). An attacker can exploit this by continuously sending unique string keys, causing the backend to allocate memory indefinitely until the application crashes.
**Learning:** Inter-Process Communication (IPC) boundaries between the frontend and backend must be treated as untrusted user input. Dynamically sized collections (like HashMaps) are highly vulnerable to resource exhaustion if arbitrary strings are accepted as keys.
**Prevention:** Always validate and allow-list string parameters against explicitly known values before inserting them into dynamically sized collections.
