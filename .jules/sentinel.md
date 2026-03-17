## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Mutex Panic Denial of Service]
**Vulnerability:** The Rust backend (`src-tauri/src/audio.rs`) used `.unwrap()` on `Mutex::lock()` results, which panics if the mutex is poisoned, crashing the thread and potentially the entire application.
**Learning:** If a thread panics while holding a mutex, the mutex becomes poisoned. Calling `.unwrap()` on a subsequent `.lock()` attempt on that poisoned mutex will cause the new thread to panic as well, creating a cascading failure and Denial of Service (DoS) vulnerability.
**Prevention:** Avoid calling `.unwrap()` on `Mutex::lock()` results. Instead, handle potential `PoisonError`s safely using methods like `.unwrap_or_else(|poisoned| poisoned.into_inner())` to recover the `MutexGuard` or a safe default to maintain availability.