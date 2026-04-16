## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-16 - Prevent Memory Exhaustion (DoS) in Tauri Commands
**Vulnerability:** Arbitrary strings from the frontend were inserted into a growing backend `HashMap` without validation, posing a memory exhaustion (DoS) risk.
**Learning:** Unvalidated frontend inputs used as keys in backend collections can lead to unbound memory growth if exploited.
**Prevention:** Always validate arbitrary string inputs from the frontend against a strict allowlist before insertion into backend state collections like `HashMap`.
