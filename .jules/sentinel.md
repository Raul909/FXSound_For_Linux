## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2026-04-28 - Recover Poisoned Mutexes to Prevent DoS
**Vulnerability:** Unhandled poisoned mutexes via `.unwrap()` can lead to permanent Denial of Service (DoS) in the audio processing thread.
**Learning:** In Rust applications, handling a poisoned Mutex by panicking or bubbling up the error does not clear the poison and can result in a permanent DoS.
**Prevention:** If data consistency allows, safely recover the `MutexGuard` using `.unwrap_or_else(|e| e.into_inner())`.
