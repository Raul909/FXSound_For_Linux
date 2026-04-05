## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2025-05-14 - Use Recursive setTimeout for Polling
**Learning:** Using setInterval for asynchronous polling operations (like Tauri invokes) can lead to overlapping calls and performance degradation if the response is delayed.
**Action:** Use a recursive setTimeout pattern instead to ensure the next request is only scheduled after the previous one completes.
