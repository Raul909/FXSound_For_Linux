## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Replace setInterval with Recursive setTimeout

**Learning:**
Using `setInterval` for asynchronous operations (like API polling or `invoke` calls) can lead to overlapping executions and performance degradation if the async operation takes longer than the interval duration.

**Action:**
Replace `setInterval` with a recursive `setTimeout` pattern. This ensures that the next execution only schedules after the previous one has fully completed, maintaining a stable execution rate and preventing queue pileups.
