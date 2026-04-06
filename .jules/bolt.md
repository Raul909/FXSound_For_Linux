## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Cache bounding rect in slider drag events
**Learning:** Calling `getBoundingClientRect()` inside high-frequency event listeners like `mousemove` causes layout thrashing and degrades UI performance during drag interactions.
**Action:** Cache the result of `getBoundingClientRect()` during the initial `mousedown` event and reuse it during subsequent `mousemove` events.
