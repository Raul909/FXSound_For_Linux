## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Overlapping Async Executions
**Learning:**
In asynchronous polling loops (e.g., calling backend via `invoke`), using `setInterval` can cause overlapping executions if the backend responds slower than the polling interval. This degrades performance and can cause memory leaks.
**Action:**
Replaced `setInterval` with a recursive `setTimeout` pattern, ensuring that the next poll is only scheduled after the previous execution has completed.
