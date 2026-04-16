## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-16 - Prevent overlapping async state updates with recursive setTimeout

**Learning:**
Using `setInterval` for asynchronous polling operations (like Tauri `invoke` calls) can lead to overlapping executions and performance degradation if the async operation takes longer than the interval.

**Action:**
Use a recursive `setTimeout` pattern for asynchronous polling to ensure the next request is only scheduled after the previous one completes.
