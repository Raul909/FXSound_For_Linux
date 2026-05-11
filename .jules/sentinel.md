## 2026-05-11 - Prevent Mutex Poisoning Denial of Service
**Vulnerability:** Mutexes wrapped around application state (like `audio_engine`) mapped poisoning errors to strings or unwrapped them, causing permanent DoS or thread crashes if any thread panicked while holding the lock.
**Learning:** Returning `Result::Err` or panicking upon discovering a poisoned Mutex permanently disables the affected feature since the lock can never be cleared.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` to safely recover the `MutexGuard` from poisoned states, clearing the poison flag and restoring functionality.
