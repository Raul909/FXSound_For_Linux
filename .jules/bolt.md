## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Prevent overlapping execution jank in async polling loops

**Learning:**
Using `setInterval` for asynchronous polling (e.g., fetching visualizer data via Tauri `invoke`) can lead to performance-degrading overlapping executions if the async operation takes longer than the interval delay. This leads to piled-up tasks and main thread jank.

**Action:**
Use a recursive `setTimeout` pattern instead. This ensures that the next execution is only scheduled *after* the previous async task has fully completed, maintaining a stable polling rate without overlap.
