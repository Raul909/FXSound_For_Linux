## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Mutex Poisoning DoS]
**Vulnerability:** The Rust backend (`src-tauri/src/audio.rs`) called `.unwrap()` directly on `Mutex` locks for the audio engine and FFT data, which meant that if a thread panicked while holding the lock, the mutex would become "poisoned," causing subsequent `.unwrap()` calls to immediately panic, leading to a cascading Denial of Service (DoS) where the entire application crashes or the audio processing thread dies permanently.
**Learning:** In real-time or continuous processing loops (like audio loops), using `.unwrap()` on shared state synchronization primitives is dangerous. Any unexpected panic in one thread can permanently break the shared state for all other threads.
**Prevention:** Avoid `.unwrap()` on `Mutex::lock()`. Instead, use `.unwrap_or_else(|poisoned| poisoned.into_inner())` or `.unwrap_or_default()` to safely recover the lock or a default value and maintain application availability, even if the state might be slightly inconsistent.
