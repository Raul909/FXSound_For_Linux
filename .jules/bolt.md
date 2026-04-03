## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-04-03 - Prevent overlapping async state updates in React polling loops
**Learning:** Using `setInterval` with asynchronous callbacks (like fetching backend data via `invoke`) can lead to performance-degrading overlapping executions if the async operation takes longer than the interval delay. This causes a pile-up of pending requests, increasing CPU load and memory usage.
**Action:** Replace `setInterval` with a recursive `setTimeout` pattern inside an asynchronous function. This ensures that the next timeout is only scheduled *after* the previous asynchronous operation has fully completed, preventing overlapping executions.
