## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Unbounded HashMap Insertion DoS]
**Vulnerability:** The `AudioEngine::set_effect` method accepted arbitrary strings from the frontend and inserted them into a backend `HashMap` without validation, allowing a malicious frontend to cause a memory exhaustion Denial of Service (DoS).
**Learning:** Arbitrary string inputs from the frontend must always be validated against a strict allowlist before being used as keys in dynamically growing backend collections.
**Prevention:** Implement allowlist validation for all dynamic keys sent over IPC before storing them in backend state.
