## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-26 - Prevent Overlapping Async Invocations
**Learning:** Using `setInterval` for repeatedly triggering asynchronous operations (like Tauri `invoke` polling) can cause overlapping executions and performance degradation if an invocation takes longer than the interval duration.
**Action:** Use a recursive `setTimeout` pattern to ensure the next invocation is only scheduled after the previous one has fully resolved, guaranteeing non-overlapping execution.
