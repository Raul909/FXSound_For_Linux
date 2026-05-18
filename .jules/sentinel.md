## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-18 - Prevent IPC Memory Exhaustion DoS
**Vulnerability:** The Tauri backend accepted untrusted `String` keys via IPC (`set_effect`) and directly inserted them into a dynamically sized `HashMap` (`self.effects`), allowing potential unbounded memory exhaustion (Denial of Service) by a compromised frontend sending arbitrary keys.
**Learning:** Always treat IPC parameters from the frontend as untrusted user input, especially when dealing with strings that are stored in collections.
**Prevention:** Validate and allow-list all string parameters against expected values before inserting them into stateful collections.
