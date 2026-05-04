## 2026-05-04 - Mutex Poisoning DoS Vulnerability
**Vulnerability:** Mutex locking code handled poisoning by panicking (`unwrap()`) or returning an error instead of recovering the lock. This allowed a single thread panic to permanently deny service (DoS) to all other components attempting to access the shared audio engine or FFT data.
**Learning:** In Rust applications, handling a poisoned Mutex by bubbling up the error or panicking does not clear the poison and can result in a permanent DoS.
**Prevention:** If data consistency allows, safely recover the `MutexGuard` using `.unwrap_or_else(|e| e.into_inner())` instead of bubbling errors or unwrapping.
