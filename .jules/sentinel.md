## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-03-25 - Prevent DoS via Cascading Panics in Audio Processing
**Vulnerability:** Calling `.unwrap()` on `Mutex` locks in real-time audio threads (`src-tauri/src/audio.rs`).
**Learning:** If a thread panics while holding the lock, the mutex becomes "poisoned". Subsequent `.unwrap()` calls by other threads will also panic, leading to a cascading failure that crashes the application (Denial of Service).
**Prevention:** Handle `PoisonError` safely to recover the lock or provide a safe default. Use `.unwrap_or_else(|poisoned| poisoned.into_inner())` to extract the `MutexGuard` and continue operating, maintaining availability.
