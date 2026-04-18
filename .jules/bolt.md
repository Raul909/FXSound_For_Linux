## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-18 - Prevent Overlapping Executions in Async React Intervals
**Learning:** Using `setInterval` for repetitive asynchronous calls (like Tauri `invoke` polling) can cause severe performance degradation if the async operation takes longer than the interval, leading to overlapping executions and an unmanageable queue of promises.
**Action:** Always use a recursive `setTimeout` pattern instead of `setInterval` for high-frequency async polling in frontend components to ensure the next request only begins after the previous one has fully resolved.
