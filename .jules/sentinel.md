## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-18 - [HashMap Unbounded Growth (DoS)]
**Vulnerability:** `AudioEngine::set_effect` accepted arbitrary effect names, inserting them unbounded into a HashMap.
**Learning:** Exposing unbounded map inserts to external inputs creates memory exhaustion risks.
**Prevention:** Always validate external keys against known constants before inserting them into stateful collections.
