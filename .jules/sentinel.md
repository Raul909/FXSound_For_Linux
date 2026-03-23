## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Fix DoS Vulnerability via Mutex Panic Cascade]
**Vulnerability:** The `unwrap()` method was called on `Mutex::lock()` results in `src-tauri/src/audio.rs` (both in the audio processing loop and when retrieving FFT data). If a thread panics while holding the mutex lock, the mutex becomes "poisoned", and subsequent `.unwrap()` calls on the lock result will cause cascading panics, leading to a Denial of Service (DoS) for the application's audio engine or visualizer functionality.
**Learning:** To prevent DoS via cascading panics in Rust applications, avoid calling `.unwrap()` on `Mutex` locks.
**Prevention:** Handle `PoisonError`s safely using methods like `.unwrap_or_else(|poisoned| poisoned.into_inner())` or `.unwrap_or_default()` to recover the `MutexGuard` or provide a safe default.
