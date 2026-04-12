## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-05-18 - Fix DoS vulnerability in effect handler
**Vulnerability:** Unbounded HashMap insertion allows memory exhaustion (Denial of Service).
**Learning:** Arbitrary string inputs from the frontend must be validated against a strict allowlist before being used as keys in dynamically growing backend collections (like `HashMap`). Otherwise, a malicious script could send infinite unique keys, consuming all available memory.
**Prevention:** Implement validation checks (e.g., `valid_keys.contains(&key)`) for all inputs used as keys in collections.
