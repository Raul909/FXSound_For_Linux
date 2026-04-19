## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-19 - [Memory Exhaustion via Unbounded HashMap]
**Vulnerability:** The `AudioEngine::set_effect` function stores effect values in a dynamic `HashMap<String, f32>`, which can grow unboundedly if arbitrary effect names are provided via the Tauri IPC command, leading to memory exhaustion (DoS).
**Learning:** Using dynamic collections for fixed, predictable state fields driven by external input is an attack vector for resource exhaustion.
**Prevention:** Use explicit structures with fixed fields for application state configurations to prevent unbound memory allocations based on external input.
