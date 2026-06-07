## 2026-06-07 - Validate IPC inputs before map insertion
**Vulnerability:** Untrusted string parameters from frontend IPC commands were directly inserted into a dynamically sized HashMap (`self.effects`) without validation.
**Learning:** In Tauri applications, treat Inter-Process Communication (IPC) command parameters from the frontend as untrusted user input. Inserting them into unbounded collections can lead to memory exhaustion (DoS) attacks.
**Prevention:** Always validate and allow-list string parameters against known valid keys (e.g., derived from frontend constants) before inserting them into dynamically sized collections.
