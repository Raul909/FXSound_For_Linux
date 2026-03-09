## 2024-05-24 - Tauri CSP Enforcement
**Vulnerability:** The Tauri configuration `src-tauri/tauri.conf.json` had its Content Security Policy (CSP) set to `null`, allowing potentially unsafe resources and scripts to be loaded.
**Learning:** A missing CSP in a desktop application exposes it to XSS and other injection attacks if any remote content is loaded.
**Prevention:** Always maintain a strict CSP. For React/Vite dev server compatibility, include `connect-src` rules for `localhost:5173` and `unsafe-inline` for scripts/styles alongside `default-src 'self'`.
