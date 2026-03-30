## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-05-30 - Fix DoS via Mutex Poisoning
**Vulnerability:** The application used `.unwrap()` on `Mutex` locks in critical real-time audio threads (`get_fft_data` and `audio_loop`).
**Learning:** Calling `.unwrap()` on a Mutex lock creates a Denial of Service (DoS) vulnerability via cascading panics. If one thread panics while holding the lock, the mutex becomes poisoned. Any subsequent thread calling `.unwrap()` on that poisoned mutex will also panic, taking down the entire application or processing loop.
**Prevention:** Avoid `.unwrap()` on Mutex locks. Instead, safely handle `PoisonError`s by using methods like `.unwrap_or_else(|poisoned| poisoned.into_inner())` to recover the `MutexGuard` and maintain availability even if a previous thread failed unexpectedly.
