## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-22 - Prevent overlapping backend polling in React
**Learning:** Using `setInterval` for asynchronous polling (like Tauri `invoke` calls) in React can cause overlapping executions if the async operation takes longer than the interval, leading to performance degradation and memory bloat.
**Action:** Always use a recursive `setTimeout` pattern for repeated asynchronous polling to ensure the next request is only scheduled after the previous one completes.
