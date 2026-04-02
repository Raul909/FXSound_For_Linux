## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Async Overlapping in React Visualizer Polling
**Learning:**
Using `setInterval` for asynchronous operations (like invoking backend commands at a high frequency) in React can cause overlapping executions if the async operation takes longer than the interval. This degrades performance by stacking pending state updates and filling the event queue unnecessarily.

**Action:**
Use a recursive `setTimeout` pattern inside an `async` function instead of `setInterval`. This ensures the next execution is only scheduled *after* the previous one fully resolves or rejects, preventing overlapping updates.
