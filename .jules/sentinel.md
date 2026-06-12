## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-12 - Prevent IPC Memory Exhaustion (DoS)
**Vulnerability:** The Tauri IPC command `set_effect` takes an untrusted string parameter from the frontend and inserts it directly into a long-lived backend `HashMap`. An attacker or compromised frontend could send thousands of unique effect names, causing unbounded memory growth and crashing the application (Denial of Service).
**Learning:** Any dynamically sized collection (like a `HashMap`) must never accept untrusted strings as keys without validation. IPC boundaries should be treated with the same suspicion as HTTP endpoints.
**Prevention:** Always validate and allow-list string parameters from IPC before storing them in backend state. Reject unknown keys immediately.
