## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-24 - Prevent Overlapping Asynchronous Polling

**Learning:**
Using `setInterval` for asynchronous polling (such as Tauri `invoke` calls) does not wait for the promise to resolve, which can lead to overlapping executions and performance degradation if the response time exceeds the interval delay.

**Action:**
Use a recursive `setTimeout` pattern to schedule the next execution only after the current asynchronous operation has completed.
