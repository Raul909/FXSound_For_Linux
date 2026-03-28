## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Cascading Panics via Mutex unwrap]
**Vulnerability:** Calling `.unwrap()` on `Mutex` locks can lead to Denial of Service (DoS) due to cascading panics if a thread panics while holding the lock.
**Learning:** In Rust, if a thread panics while holding a `Mutex`, the mutex becomes poisoned. Calling `unwrap()` on it subsequently will cause other threads to panic as well, bringing down the application.
**Prevention:** Always handle `PoisonError` gracefully to recover the lock or a safe default instead of crashing, for example using `.unwrap_or_else(|poisoned| poisoned.into_inner())`.
