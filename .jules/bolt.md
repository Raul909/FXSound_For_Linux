## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2025-05-14 - Optimize Tauri Polling Interval

**Learning:** Using `setInterval` for asynchronous polling (like `invoke("get_visualizer_data")`) can cause overlapping calls and performance degradation if the response takes longer than the interval.
**Action:** Always use a recursive `setTimeout` pattern to ensure the previous call completes before scheduling the next one.
