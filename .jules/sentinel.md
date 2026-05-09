
## 2026-05-09 - Fix Mutex Poisoning DoS
**Vulnerability:** Tauri command handlers handled poisoned `AppState` mutexes by returning errors via `.map_err(|e| e.to_string())?`. This is a Denial of Service (DoS) vulnerability, as it fails to clear the poison state, causing all subsequent requests to fail permanently.
**Learning:** Returning an error on a poisoned mutex propagates the broken state, essentially disabling the affected endpoints and making the app vulnerable to continuous DoS if a panic happens on a single thread.
**Prevention:** Safely recover the `MutexGuard` using `.unwrap_or_else(|e| e.into_inner())` to resume execution gracefully instead of bailing out on poison errors.
