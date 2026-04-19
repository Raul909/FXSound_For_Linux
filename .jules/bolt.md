## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-19 - Prevent Overlapping Executions with Recursive setTimeout

**Learning:**
Using `setInterval` with asynchronous operations (like Tauri `invoke` polling) can cause overlapping executions and performance degradation in the React frontend if the backend call takes longer than the interval. This creates a backlog of unresolved promises.

**Action:**
Replace `setInterval` with a recursive `setTimeout` pattern to ensure the next poll only happens after the current asynchronous operation finishes, preventing memory leaks and CPU spikes.
