## 2024-05-02 - Mutex Poisoning Denial of Service
**Vulnerability:** The application handles poisoned Mutexes by either panicking (`.unwrap()`) or returning an error via `map_err`, leading to a permanent Denial of Service (DoS) where the audio engine or application features become permanently unusable.
**Learning:** In long-running applications like audio engines, standard Mutex poisoning handling can result in a state where a single thread panic permanently locks out all other threads from accessing shared state.
**Prevention:** Always use `.unwrap_or_else(|e| e.into_inner())` on shared state mutexes (when data consistency allows) to safely recover the `MutexGuard` and ensure continuous operation.
