## 2026-05-08 - Mitigate Mutex Poisoning DoS
**Vulnerability:** Mutexes holding the application state (AudioEngine and fft_data) handle lock poisoning by panicking (`unwrap()`) or returning errors (`map_err`), leading to a permanent Denial of Service (DoS) where the app or audio processing drops permanently.
**Learning:** Mutex poisoning in Rust occurs when a thread panics while holding the lock. If the lock is accessed again, it will return a PoisonError. Unwrapping or propagating this error indefinitely halts progress.
**Prevention:** Always safely recover poisoned mutexes using `.unwrap_or_else(|e| e.into_inner())` if the contained state remains valid or can be reinitialized, ensuring continuous application availability.
