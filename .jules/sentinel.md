## 2026-05-06 - Fix Poisoned Mutex DoS
**Vulnerability:** Poisoned Mutex DoS via incorrect mutex lock handling in Rust, causing the thread to crash or fail indefinitely.
**Learning:** Returning an error (`.map_err(...)`) or panicking (`.unwrap()`) upon mutex poisoning fails to clear the poison, leading to a permanent DoS.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` to safely recover the MutexGuard if data consistency allows.
