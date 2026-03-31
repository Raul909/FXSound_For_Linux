## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-19 - [DoS via Cascading Panics on Mutex]
**Vulnerability:** The application was calling `.unwrap()` on `Mutex` locks in critical sections (like `get_fft_data` and the main `audio_loop`). If a thread panicked while holding the lock, the Mutex would become "poisoned," causing subsequent `.unwrap()` calls by other threads to also panic, leading to a Denial of Service (DoS).
**Learning:** Using `.unwrap()` on Mutexes in Rust creates a cascading failure point, where a single localized panic can crash the entire application by continuously poisoning lock attempts.
**Prevention:** Avoid calling `.unwrap()` on `Mutex` locks. Instead, safely handle potential `PoisonError`s using methods like `.unwrap_or_else(|e| e.into_inner())` or `.unwrap_or_default()` to recover the lock and maintain availability.
