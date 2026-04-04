## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-05-18 - Prevent overlapping executions in async polling
**Learning:** Using `setInterval` to repeatedly trigger asynchronous operations (like Tauri `invoke` calls for fetching visualizer data) can lead to overlapping executions and performance degradation if the async operation occasionally takes longer than the interval duration.
**Action:** Replace `setInterval` with a recursive `setTimeout` pattern, where the next timeout is only scheduled *after* the previous asynchronous operation has fully completed.
