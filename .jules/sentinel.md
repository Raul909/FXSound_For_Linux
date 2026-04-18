## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-18 - [Fix DoS Vulnerability in AudioEffects]
**Vulnerability:** The `AudioEngine` used an unbounded `HashMap` for storing audio effects. The `set_effect` method allowed insertion of arbitrary string keys, which could be abused to cause unbounded memory growth and eventually a Denial of Service (DoS) via Out of Memory (OOM).
**Learning:** Using dynamic collections like `HashMap` for state that is populated by external input without validation or limits exposes the application to memory exhaustion vulnerabilities. Fixed, bounded structures should be preferred for finite configurations.
**Prevention:** Use explicitly defined structs with fixed fields to store configurations, and `match` statements to handle input safely, ignoring or rejecting unknown keys.
