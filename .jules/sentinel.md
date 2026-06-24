## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-13 - Fix NaN Propagation and Log Injection DoS
**Vulnerability:** The backend Tauri commands `set_eq_band` and `set_effect` receive `f32` numbers and a `String` directly from the frontend without validation. A malicious or buggy frontend could send `NaN` as `gain`, causing Biquad filter coefficient calculations (`10.0f32.powf(NaN)`) to output `NaN`, permanently silencing the application. Additionally, an unbounded string could cause excessive memory allocation, or contain CRLF for log injection.
**Learning:** Never trust frontend data implicitly, even if it's "just the local UI". Data types like `f32` can carry invalid states (`NaN`, `Infinity`) that propagate through math operations and cause silent, unrecoverable failures.
**Prevention:** Always validate floating point numbers using `.is_finite()` immediately upon entry at the RPC/IPC boundary. Enforce strict length limits and character constraints on strings, especially if used as map keys or in logging.
