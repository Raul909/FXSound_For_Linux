## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Cache bounding rect in slider components
**Learning:** To prevent layout thrashing in high-frequency React event listeners (e.g., mousemove for UI sliders), cache the output of getBoundingClientRect() during the initial mousedown event instead of recalculating it on every pixel moved.
**Action:** Move getBoundingClientRect() calls from the move event handler or translation functions to the initial mousedown event handler and pass the cached rect.
