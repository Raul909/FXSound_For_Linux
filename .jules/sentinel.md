## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-23 - [Mutex Poisoning Denial of Service]
**Vulnerability:** The audio engine used `.unwrap()` on Mutex locks (`engine` and `fft_data`). If a panic occurred while a lock was held, the mutex would become poisoned, and subsequent `.unwrap()` calls would panic, permanently crashing the audio loop (Denial of Service).
**Learning:** Panicking on poisoned mutexes in long-running background threads creates brittle applications susceptible to DoS.
**Prevention:** Recover from poisoned mutexes safely using `.unwrap_or_else(|e| e.into_inner())` when data consistency allows, ensuring continuous operation even after transient thread panics.
