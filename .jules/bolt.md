## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-12 - Avoid setInterval for Async Polling
**Learning:** Using `setInterval` for asynchronous operations like Tauri API calls can cause overlapping executions and performance degradation if the operation takes longer than the interval. This codebase's React frontend should avoid this pattern.
**Action:** Always use a recursive `setTimeout` pattern when repeatedly triggering asynchronous operations to ensure the next iteration only begins after the previous one has completed.
