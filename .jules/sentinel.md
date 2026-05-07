## 2026-05-07 - Persistent DoS via Poisoned Mutexes
**Vulnerability:** Persistent Denial of Service (DoS) caused by poisoned Mutexes in Rust.
**Learning:** When a thread panics while holding a Mutex, the Mutex becomes poisoned. Panicking via `.unwrap()` or returning errors via `map_err` upon acquiring a poisoned Mutex fails to clear the poison state, leading to a permanent DoS.
**Prevention:** If the data structure inside the Mutex remains consistent even after a panic, safely recover the lock using `.unwrap_or_else(|e| e.into_inner())` instead of propagating the error or panicking.
