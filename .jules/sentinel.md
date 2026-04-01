## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [DoS via Cascading Panics]
**Vulnerability:** Calling `.unwrap()` on Mutex locks can lead to Denial of Service (DoS) through cascading panics if a thread panics while holding the lock, poisoning the Mutex.
**Learning:** In long-running background threads (like the audio processing loop), a panic from a poisoned Mutex will crash the thread permanently, stopping core functionality.
**Prevention:** Always handle `PoisonError` safely using methods like `.unwrap_or_else(|p| p.into_inner())` to recover the `MutexGuard` and maintain application availability.
