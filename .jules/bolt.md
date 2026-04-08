## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Overlapping Executions in React Polling
**Learning:** Using `setInterval` for asynchronous operations (like Tauri invokes) in React effects can lead to overlapping executions and performance degradation if the async task takes longer than the interval.
**Action:** Use a recursive `setTimeout` pattern instead to ensure the next polling operation only begins after the previous one completes.
