## 2026-05-12 - Fix Mutex Poisoning Denial of Service (DoS)
**Vulnerability:** Mutex locks in Tauri command handlers and audio engine use `.unwrap()` or `.map_err(|e| e.to_string())?`. If a thread panics while holding the lock, the mutex becomes permanently "poisoned", causing all subsequent lock attempts to fail, resulting in an unrecoverable Denial of Service (DoS) requiring a restart.
**Learning:** Rust's Mutex poisoning is a safety feature, but failing to clear it creates a permanent DoS state in long-running services. Panics in thread boundaries shouldn't permanently take down shared state.
**Prevention:** Use `.unwrap_or_else(|e| e.into_inner())` on `Mutex::lock()` results to safely recover the lock guard and continue functioning even if the previous thread panicked.
## 2024-06-20 - Prevent Unbounded HashMap Growth via Untrusted Input
**Vulnerability:** The `set_effect` backend command accepted arbitrary string keys from the frontend without validation and inserted them directly into a HashMap, creating a potential memory exhaustion (DoS) vulnerability.
**Learning:** Even though Tauri IPC limits individual message sizes, malicious or buggy frontend code could repeatedly send unique string keys, leading to uncontrolled memory growth in the backend state.
**Prevention:** Always validate and allowlist untrusted input keys before using them as identifiers in dynamically sized backend data structures (like HashMaps).
