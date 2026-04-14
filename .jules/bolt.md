## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Prevent Visualizer Polling Overlap
**Learning:** Using `setInterval` for asynchronous operations like polling the Tauri backend can cause overlapping executions and layout thrashing if responses are delayed.
**Action:** Use a recursive `setTimeout` pattern instead to ensure the next request is only scheduled after the previous one completes.
