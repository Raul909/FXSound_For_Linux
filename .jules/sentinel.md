## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-17 - [Unbounded HashMap Insertion DoS]
**Vulnerability:** The `AudioEngine::set_effect` method accepted arbitrary string keys from the frontend and inserted them directly into a Rust `HashMap` without validation.
**Learning:** Allowing arbitrary, unvalidated strings from the frontend to be used as keys in backend collections (like HashMaps) can lead to memory exhaustion (Denial of Service) attacks as the collection grows unbounded.
**Prevention:** Always validate arbitrary string inputs from the frontend against a strict allowlist before inserting them into dynamically growing backend data structures.
