## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2024-05-24 - [Unbounded HashMap Insertion via Frontend Input]
**Vulnerability:** The `AudioEngine::set_effect` backend command accepted arbitrary effect names directly from the React frontend, inserting them directly into a `HashMap<String, f32>` without validation. This allowed an attacker to send random or maliciously generated effect names to exhaust memory (DoS vulnerability).
**Learning:** In a typical web backend, endpoints sanitize parameters; however, in a Tauri app with an IPC bridge, passing arbitrary `String` keys to backend commands can still lead to unbounded memory allocation if the data structure continuously grows.
**Prevention:** Always validate arbitrary string inputs from the frontend against a strict allowlist (e.g., predefined valid effects) before inserting them into dynamically expanding backend data structures like HashMaps or Vecs.
