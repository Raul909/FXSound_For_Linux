## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-23 - Prevent overlapping async polling

**Learning:**
Using `setInterval` with asynchronous operations (like Tauri's `invoke` polling) can cause overlapping executions if the async operation takes longer than the interval delay. This leads to piled-up calls and performance degradation.

**Action:**
Use a recursive `setTimeout` pattern instead of `setInterval` to ensure the next iteration only begins after the current one has fully completed.
