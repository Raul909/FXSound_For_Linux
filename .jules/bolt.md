## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-19 - Replace setInterval with recursive setTimeout for async polling

**Learning:**
Using `setInterval` for asynchronous operations like Tauri `invoke` polling can lead to overlapping executions and UI lag if the async task takes longer than the interval duration.

**Action:**
Use a recursive `setTimeout` pattern to schedule the next poll only after the current asynchronous execution has fully completed.
