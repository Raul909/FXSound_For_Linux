## 2026-06-08 - Prevent IPC Memory Exhaustion (DoS)
**Vulnerability:** The Tauri `set_effect` command accepted any string for the `effect` parameter and inserted it directly into a Rust `HashMap`. A malicious frontend or compromised renderer could send thousands of unique effect names, causing unbounded memory allocation and leading to a Denial of Service (DoS) via memory exhaustion.
**Learning:** In Tauri applications, Inter-Process Communication (IPC) command parameters must be treated as untrusted user input. Dynamically sized collections like `HashMap` in the backend are vulnerable to exhaustion if keys are not validated.
**Prevention:** Always implement an explicit allow-list for string parameters at the IPC boundary before using them as keys in dynamically sized collections.
