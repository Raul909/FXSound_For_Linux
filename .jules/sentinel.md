## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-06-21 - Fix Unbounded State HashMap (DoS)
**Vulnerability:** The `set_effect` Tauri command accepted arbitrary `effect` string keys from the frontend and inserted them into the backend `HashMap` without validation. A malicious frontend could send millions of unique keys, causing unbounded memory allocation and crashing the Rust backend (OOM/DoS).
**Learning:** Never trust frontend keys for state Maps or HashMaps. Always validate them against an allowlist of expected keys before insertion.
**Prevention:** Use `matches!` or a predefined `enum` to validate dynamic string keys coming from the frontend before inserting them into a persistent state `HashMap`.
