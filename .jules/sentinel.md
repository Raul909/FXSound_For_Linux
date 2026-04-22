## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-22 - Fix Mutex Poisoning Denial of Service
**Vulnerability:** Unhandled Mutex poisoning via `.unwrap()` causes panic cascades (DoS) in the audio processing loop and visualizer endpoints.
**Learning:** In Rust applications, when a thread panics while holding a Mutex, it becomes "poisoned". Calling `.unwrap()` on a poisoned Mutex causes subsequent threads to panic, resulting in a persistent DoS. Simply bubbling up the error does not fix the DoS.
**Prevention:** To safely recover from a poisoned Mutex and maintain application uptime, handle the `PoisonError` by extracting the inner `MutexGuard` using `.unwrap_or_else(|e| e.into_inner())` instead of panicking.
