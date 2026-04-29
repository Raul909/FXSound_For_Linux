## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2026-04-29 - Mutex Poisoning DoS
**Vulnerability:** unwrap() on Mutex lock results in panic on poisoned mutex, leading to DoS.
**Learning:** To safely handle Mutex poisoning without crashing, extract the MutexGuard.
**Prevention:** Use unwrap_or_else(|e| e.into_inner()) instead of unwrap().
