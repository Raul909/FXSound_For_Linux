## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-04-14 - Fix Unbounded Memory Growth in HashMap
**Vulnerability:** The backend `AudioEngine::set_effect` method accepted arbitrary strings from the frontend, inserting them into a `HashMap` without validation, allowing unbounded memory growth (Denial of Service).
**Learning:** Always validate arbitrary string inputs from the frontend against a strict allowlist before using them as keys in dynamically growing backend collections in Tauri applications.
**Prevention:** Implement input validation using an allowlist for all commands that accept dynamic string keys from IPC boundaries.
