## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-07 - [Unbounded HashMap Insertion DoS in Tauri Command]
**Vulnerability:** The `AudioEngine::set_effect` function in `src-tauri/src/audio.rs` accepted arbitrary string keys from the frontend and inserted them directly into a `HashMap` without validation.
**Learning:** This exposes the backend to memory exhaustion (Denial of Service) attacks where a malicious frontend or script could insert unbounded arbitrary keys, infinitely growing the HashMap.
**Prevention:** Always validate arbitrary string inputs from the frontend against a strict allowlist before using them as keys in dynamically growing backend collections (e.g., HashMaps).
