## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-21 - Prevent overlapping executions with recursive setTimeout
**Learning:** Using `setInterval` for asynchronous operations like Tauri IPC calls can cause overlapping executions and performance degradation in the React frontend if operations take longer than the interval.
**Action:** Use a recursive `setTimeout` pattern instead of `setInterval` when repeatedly triggering asynchronous operations to ensure the previous operation completes before the next one is queued.
