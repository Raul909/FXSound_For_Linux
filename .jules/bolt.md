## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Tauri/React Visualizer Overlapping Invokes
**Learning:** Using `setInterval` to continuously poll the Tauri backend via IPC (e.g., `invoke("get_visualizer_data")`) is a dangerous performance anti-pattern. If the backend process takes longer than the interval (e.g., waiting for audio buffers), multiple overlapping calls stack up. This saturates the IPC channel, causing massive performance degradation, visualizer stalling, and audio desync.
**Action:** Always use recursive `setTimeout` for asynchronous polling loops (especially `invoke` calls) in React to guarantee that the previous backend response has completely resolved before the next one is scheduled.
