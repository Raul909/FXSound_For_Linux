## 2024-05-24 - Tauri CSP Configuration
**Vulnerability:** The Content Security Policy (CSP) in `tauri.conf.json` was set to `null`, completely disabling CSP protections and leaving the application vulnerable to Cross-Site Scripting (XSS).
**Learning:** Tauri's `security.csp` configuration accepts `null` to disable CSP, which is highly dangerous for desktop applications rendering HTML/JS.
**Prevention:** Always maintain a strict CSP such as `default-src 'self'` in `tauri.conf.json` and avoid setting it to `null`.
