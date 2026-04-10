## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-04-10 - Unbounded IPC HashMap Insertion
**Vulnerability:** The `set_effect` Tauri command accepted arbitrary string keys from the frontend and inserted them into a backend `HashMap`, allowing potential memory exhaustion (DoS).
**Learning:** In Tauri applications, all IPC inputs (like command arguments) must be treated as untrusted.
**Prevention:** Always validate arbitrary string inputs from the frontend against a strict allowlist before using them as keys in dynamically growing backend collections.
