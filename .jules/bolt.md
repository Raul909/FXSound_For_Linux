## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-05-14 - Cache getBoundingClientRect on drag events
**Learning:** Calling `getBoundingClientRect()` inside high-frequency `mousemove` events causes layout thrashing and performance degradation.
**Action:** Cache the bounding rectangle during the initial `mousedown` event and reuse it during subsequent `mousemove` updates.
