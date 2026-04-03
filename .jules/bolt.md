## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Layout Thrashing in Slider Event Listeners

**Learning:**
Recalculating layout properties like `getBoundingClientRect()` within high-frequency event listeners (like `mousemove` during drag operations) forces the browser to synchronously recalculate layout on every frame, causing a performance bottleneck known as "layout thrashing".

**Action:**
Cache the layout measurements (e.g., the bounding rect) once during the initiating event (like `mousedown`) and reuse the cached values in the continuous event listener loop to avoid triggering redundant layout calculations.
