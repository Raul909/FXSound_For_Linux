## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [Unbounded HashMap Insertion DoS]
**Vulnerability:** The `AudioEngine::set_effect` method accepted arbitrary effect names as strings from the frontend and inserted them directly into a `HashMap` without validation. A malicious actor or compromised frontend could exhaust backend memory by continuously sending random effect names.
**Learning:** In applications bridging frontend boundaries (like Tauri or Electron), all arbitrary string inputs must be strictly validated against an allowlist before being used as keys in dynamically growing backend collections.
**Prevention:** Implement strict allowlist validation in Tauri command handlers or underlying backend methods to ensure only recognized keys are inserted into data structures like HashMaps.
