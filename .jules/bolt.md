## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-20 - Prevent overlapping executions
**Learning:** Using setInterval for repeated asynchronous operations like Tauri invoke polling can lead to overlapping executions, memory exhaustion, and performance degradation if the operation takes longer than the interval.
**Action:** Use a recursive setTimeout pattern instead to ensure the next execution only starts after the current one has finished.
