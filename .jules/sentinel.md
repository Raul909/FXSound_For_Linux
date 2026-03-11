## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Denial of Service via Mutex Cascading Panics]
**Vulnerability:** Calling `.unwrap()` on `Mutex` lock results in `src-tauri/src/audio.rs`. If a thread panics while holding the lock, the mutex becomes poisoned. Subsequent threads calling `.unwrap()` on the poisoned mutex will also panic, causing a cascading failure that brings down the audio processing engine.
**Learning:** In long-running Rust applications (like a Tauri backend audio loop), unhandled `PoisonError`s on shared state (`Arc<Mutex<T>>`) create a Denial of Service risk.
**Prevention:** Always handle `PoisonError` gracefully to maintain availability. Use `.unwrap_or_else(|poisoned| poisoned.into_inner())` to recover the `MutexGuard` and continue processing, or propagate the error properly.
