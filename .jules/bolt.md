## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2024-04-07 - Use recursive setTimeout instead of setInterval for polling
**Learning:** In React components that repeatedly trigger asynchronous operations (like Tauri `invoke` polling for visualizer data), using `setInterval` can cause overlapping executions and performance degradation if the async operation takes longer than the interval.
**Action:** Use a recursive `setTimeout` pattern instead of `setInterval` to ensure the next iteration only schedules after the previous async operation completes or hits the next tick safely, preventing queue buildup.
