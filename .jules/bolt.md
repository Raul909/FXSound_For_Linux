## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-28 - Prevent Async Overlapping in React State

**Learning:**
When polling asynchronous Tauri endpoints (e.g., `invoke('get_visualizer_data')`) in the React frontend, using `setInterval` can cause a backlog of overlapping executions if the backend takes longer than the interval to respond, degrading performance.

**Action:**
Replaced `setInterval` with a recursive `setTimeout` pattern to ensure the next request is only scheduled after the previous one completes.
