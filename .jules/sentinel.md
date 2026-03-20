## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2026-03-20 - [Mutex Poisoning DoS Risk in Real-Time Audio]
**Vulnerability:** Calling `.unwrap()` on `Mutex` locks in Rust (e.g., `fft_data.lock().unwrap()`) causes a thread panic if the mutex becomes "poisoned" (meaning another thread panicked while holding the lock). In a multi-threaded audio processing environment, this can lead to cascading panics and a complete application crash (Denial of Service).
**Learning:** Using `unwrap()` on locks in production Rust applications creates fragility, especially in concurrent systems where one failing thread shouldn't bring down the entire application or audio engine.
**Prevention:** Always handle `PoisonError`s gracefully using methods like `.unwrap_or_else(|poisoned| poisoned.into_inner())` or `.unwrap_or_default()` to recover the lock or a safe default state, ensuring continued availability.
