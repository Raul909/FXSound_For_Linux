## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-17 - Prevent Overlapping Async Calls in React Frontend
**Learning:** Using `setInterval` to repeatedly trigger asynchronous operations (like Tauri `invoke` polling) can cause overlapping executions and performance degradation if the operation takes longer than the interval.
**Action:** Always use a recursive `setTimeout` pattern instead of `setInterval` when repeatedly polling asynchronous operations to ensure the previous call completes before the next one is queued.
