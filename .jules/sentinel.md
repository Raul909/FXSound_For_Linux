## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.

## 2026-05-16 - Prevent Memory Exhaustion via IPC Command Validation
**Vulnerability:** The Tauri backend accepts arbitrary string keys from the frontend for audio effects, inserting them directly into a dynamically sized HashMap. An attacker could send thousands of unique keys via IPC commands, leading to unbounded memory allocation and eventual Denial of Service (DoS).
**Learning:** Inter-Process Communication (IPC) command parameters from the frontend must be treated as untrusted user input, even in desktop applications. Dynamically sized collections (like HashMaps) are vulnerable to memory exhaustion attacks if unbounded inserts are permitted.
**Prevention:** Always validate and allow-list string parameters at the IPC boundary (e.g., in Tauri command handlers) before passing them to internal state engines or collections.
