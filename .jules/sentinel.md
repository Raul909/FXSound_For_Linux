## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2025-05-18 - [Denial of Service via Mutex unwrap]
**Vulnerability:** Calling `.unwrap()` on `Mutex::lock()` in the real-time Rust audio engine and visualizer backend.
**Learning:** In Rust, if a thread panics while holding a `Mutex` lock, the mutex becomes "poisoned". Any subsequent attempt by other threads to lock it and call `.unwrap()` will also panic, causing a cascading failure that results in a Denial of Service (DoS) for the application's audio processing or visualizer polling.
**Prevention:** Avoid `.unwrap()` on `Mutex` locks in high-availability loops. Instead, handle the `PoisonError` safely by recovering the lock data with `.unwrap_or_else(|poisoned| poisoned.into_inner())` or `.unwrap_or_default()` to maintain application stability.
