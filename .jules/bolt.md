## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-24 - Prevent Overlapping Async Calls in Frontend Polling

**Learning:** Using `setInterval` for repeatedly triggering asynchronous operations (like Tauri backend polling via `invoke`) can lead to overlapping executions and performance degradation if the async task takes longer than the interval.
**Action:** Replaced `setInterval` with a recursive `setTimeout` pattern in `Visualizer.jsx` to ensure that the next polling cycle only begins after the previous one has fully completed.
