## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.

## 2026-04-26 - [Unbounded HashMap DoS Vulnerability]
**Vulnerability:** The `AudioEngine` in `src-tauri/src/audio.rs` stored effect values in a `HashMap<String, f32>`, allowing malicious or rapid, unbounded insertions of arbitrary string keys via the `set_effect` Tauri command, potentially causing memory exhaustion (Denial of Service).
**Learning:** Using dynamic collections (like `HashMap`) for bounded data exposes the application to resource exhaustion if user input is not strictly validated or if the collection can grow indefinitely.
**Prevention:** Replace dynamic collections with statically sized data structures, such as a custom struct with explicitly typed fields (e.g., `AudioEffects`), to enforce bounded memory allocation and prevent DoS attacks.
