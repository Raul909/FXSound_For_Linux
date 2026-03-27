## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Fix DoS vulnerability via unhandled Mutex panics]
**Vulnerability:** The Rust backend used `Mutex::lock().unwrap()` which could lead to cascading panics across threads (Denial of Service) if one thread panicked while holding the Mutex.
**Learning:** Using `unwrap()` on a `Mutex::lock()` Result will trigger a panic if the lock is poisoned. This turns a single thread failure into a global app failure.
**Prevention:** Handle potential `PoisonError`s gracefully using `.unwrap_or_else(|p| p.into_inner())` to recover the `MutexGuard` safely and maintain availability.
