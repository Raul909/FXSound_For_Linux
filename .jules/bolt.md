## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Use recursive setTimeout for async polling in React

**Learning:**
Using `setInterval` for async operations (like polling a backend `invoke`) can lead to overlapping executions and performance degradation in React if the async task takes longer than the interval duration.

**Action:**
Replaced `setInterval` with a recursive `setTimeout` pattern that waits for the previous execution to complete before scheduling the next one, ensuring stable performance without overlapping calls.
