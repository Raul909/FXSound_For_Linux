## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize Asynchronous Polling Loops

**Learning:**
Using `setInterval` for asynchronous polling (like `invoke("get_visualizer_data")`) can cause overlapping executions and memory leaks if the backend takes longer to respond than the interval duration. The interval schedules a new call regardless of whether the previous one finished.

**Action:**
Replaced `setInterval` with a recursive `setTimeout` pattern in `Visualizer.jsx`. This ensures the next request is only scheduled after the previous one fully resolves or rejects, preventing performance degradation and event loop blockage.
