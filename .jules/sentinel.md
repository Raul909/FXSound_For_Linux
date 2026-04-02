## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Cascading Panics via Mutex Unwrap]
**Vulnerability:** The application used `.unwrap()` on Mutex locks (`self.fft_data.lock().unwrap()` and `engine.lock().unwrap()`) within critical, high-frequency audio processing and visualizer data retrieval paths.
**Learning:** If a thread panics while holding a Mutex lock, the Mutex becomes "poisoned". Subsequent attempts by other threads to lock the Mutex and `.unwrap()` the result will cause those threads to panic as well, leading to a cascading failure and effectively causing a Denial of Service (DoS) of the application's core functionality.
**Prevention:** To prevent this, always handle `PoisonError` gracefully instead of unwrapping. Use methods like `.unwrap_or_else(|p| p.into_inner())` or `.unwrap_or_default()` to recover the lock or a safe default, maintaining application availability even after a thread panic.
