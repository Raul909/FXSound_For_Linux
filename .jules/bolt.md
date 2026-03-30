## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-10-24 - Cache bounding rect in slider drag handlers
**Learning:** Calling `getBoundingClientRect()` inside high-frequency event handlers like `mousemove` causes severe layout thrashing and performance degradation, particularly in interactive UI components like sliders.
**Action:** Cache the bounding rect on the initial `mousedown` event and use the cached values for all subsequent `mousemove` calculations during the drag operation to prevent unnecessary reflows.
