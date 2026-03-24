## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2025-05-15 - React Event Listener Layout Thrashing

**Learning:**
Binding `getBoundingClientRect()` synchronously within high-frequency React event listeners (like `mousemove` on custom sliders) causes severe layout thrashing because the DOM recalculates styling on every pixel moved.

**Action:**
To eliminate redundant recalculations without breaking slider math, explicitly cache the bounding rect during the initial `mousedown` event and pass it as an argument into the position-to-value conversion functions.
