## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-05-18 - Prevent DoS via Cascading Panics from Mutex Poisoning
**Vulnerability:** The application used `.unwrap()` on `Mutex::lock()` results in the main audio processing loop (`engine.lock().unwrap()`) and when fetching visualizer data (`fft_data.lock().unwrap()`).
**Learning:** If a thread panics while holding a `Mutex`, the lock becomes "poisoned". Any subsequent calls to `lock().unwrap()` by other threads will also panic, causing a cascading failure that crashes the entire application, leading to a Denial of Service (DoS).
**Prevention:** To maintain application availability, always recover from `PoisonError`s gracefully when locking `Mutex`es, for example by using `.unwrap_or_else(|p| p.into_inner())` or `.unwrap_or_default()`, depending on the specific use case and risk of working with potentially inconsistent state.
