## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2025-04-13 - [HashMap Unbounded Growth DoS]
**Vulnerability:** The `AudioEngine::set_effect` function in `src-tauri/src/audio.rs` blindly inserts arbitrary user-provided effect string keys into a `HashMap` without any validation.
**Learning:** This is a memory exhaustion (Denial of Service) vulnerability. A malicious or compromised frontend could repeatedly invoke `set_effect` with unique random string keys, causing the backend's `HashMap` to grow indefinitely until the application runs out of memory and crashes.
**Prevention:** Always validate dynamically generated or untrusted string inputs against a strict allowlist before using them as keys in dynamically growing collections like `HashMap`.
