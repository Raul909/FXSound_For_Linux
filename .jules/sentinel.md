## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - Prevent DoS from Mutex poisoning panics
**Vulnerability:** Calls to `unwrap()` on `Mutex::lock()` returns in the Rust backend code. This means if any thread panics while holding the lock, subsequent attempts to acquire the lock will panic due to lock poisoning, causing a cascading application failure (Denial of Service).
**Learning:** In highly concurrent real-time audio systems, failing gracefully is crucial. `Mutex` locks should recover rather than panic.
**Prevention:** Avoid calling `.unwrap()` on `Mutex::lock()`. Use `.unwrap_or_else(|poisoned| poisoned.into_inner())` to safely recover the `MutexGuard` state even if the lock is poisoned, maintaining service availability.
