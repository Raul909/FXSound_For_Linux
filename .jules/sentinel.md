## 2024-05-18 - [Tauri CSP Misconfiguration]
**Vulnerability:** The Tauri configuration (`src-tauri/tauri.conf.json`) had `app.security.csp` set to `null`, completely disabling Content Security Policy protections.
**Learning:** Setting CSP to null in Tauri exposes the application to Cross-Site Scripting (XSS) risks by allowing execution of unauthorized scripts and loading of resources from any origin.
**Prevention:** Always define a strict baseline CSP, such as `"default-src 'self'; style-src 'self' 'unsafe-inline'"`, in Tauri applications to restrict script execution and resource loading to trusted sources.
## 2024-05-24 - [CRITICAL] Fix unhandled panics on poisoned Mutexes
**Vulnerability:** Rust `Mutex::lock()` returns a `PoisonError` if a panic occurs while another thread holds the lock. Calling `.unwrap()` on the result causes a cascading panic in the current thread, leading to a Denial of Service (DoS) in long-running applications like `fxsound-linux`.
**Learning:** In real-time or multi-threaded Rust applications, relying on `.unwrap()` for shared state (`Mutex`, `RwLock`) introduces brittleness and security risks, particularly when the state is accessed across different event loops (e.g., UI commands and background DSP processors).
**Prevention:** Always handle `PoisonError` gracefully by using `.unwrap_or_else(|poisoned| poisoned.into_inner())` or `.unwrap_or_default()` to maintain system availability and gracefully recover the guard.
