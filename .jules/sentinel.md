## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2026-04-27 - Mutex Poisoning DoS in Audio Pipeline
**Vulnerability:** Application uses `.unwrap()` and `.map_err(|e| e.to_string())?` on long-lived `Mutex` locks (e.g., `AudioEngine` and `fft_data`), causing permanent Denial of Service (DoS) across threads if any thread panics while holding the lock.
**Learning:** Bubbling up or panicking on a poisoned mutex does not clear the poison state in Rust. Subsequent lock attempts will continue to fail permanently.
**Prevention:** Always use `.unwrap_or_else(|e| e.into_inner())` to safely recover the `MutexGuard` when data consistency allows it, rather than propagating the error or panicking.
