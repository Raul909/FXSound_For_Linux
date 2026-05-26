## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-26 - Fix IPC Memory Exhaustion (DoS)
**Vulnerability:** The `set_effect` command inserted untrusted string keys directly from IPC into a `HashMap<String, f32>` without validation.
**Learning:** In Tauri applications, all frontend inputs over IPC must be treated as untrusted. Dynamically sized collections like HashMaps can be exploited to cause Out-of-Memory (OOM) crashes if populated with unvalidated keys.
**Prevention:** Always validate and allow-list string parameters before inserting them into dynamically sized collections to prevent memory exhaustion attacks.
