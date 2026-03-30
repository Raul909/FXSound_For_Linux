## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-05-23 - Prevent overlapping async IPC calls

**Learning:**
Using `setInterval` for asynchronous polling (like fetching visualizer data via Tauri's `invoke`) is a performance anti-pattern. If the async call takes longer than the interval duration, promises can pile up and overlap, leading to degraded performance and potential memory leaks.

**Action:**
Replaced `setInterval` with a recursive `setTimeout` pattern in `src/components/Visualizer.jsx`. This ensures that the next poll is only scheduled after the previous async operation has fully completed, preventing overlapping executions.
