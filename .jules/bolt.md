## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Prevent Layout Thrashing in React Event Listeners
**Learning:** Calling `getBoundingClientRect()` inside high-frequency event listeners (like `mousemove` on custom sliders) causes layout thrashing and performance degradation, especially when many elements are rendered or updated.
**Action:** Always cache the output of `getBoundingClientRect()` during the initial interaction event (e.g., `mousedown`) and reuse the cached `rect` inside subsequent high-frequency events, rather than recalculating it on every pixel moved.
