## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2025-03-10 - [Denial of Service via Cascading Mutex Panics]
**Vulnerability:** The audio processing engine (`src-tauri/src/audio.rs`) called `.unwrap()` when acquiring `std::sync::Mutex` locks for the shared audio engine state and FFT visualizer data.
**Learning:** If a single thread panics while holding a standard library Mutex, the Mutex becomes "poisoned". Subsequent calls to `.lock().unwrap()` by other threads will also panic. In an application with continuous background threads (like real-time audio processing) and frequent frontend polling via Tauri commands, a single panic can cause a cascading failure, crashing the entire application (DoS).
**Prevention:** Instead of blindly unwrapping Mutex locks, handle `PoisonError` gracefully using `.unwrap_or_else(|poisoned| poisoned.into_inner())` to safely recover the `MutexGuard` and preserve application availability.