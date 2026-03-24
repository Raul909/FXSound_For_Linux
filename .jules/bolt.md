## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-03-24 - Prevent Layout Thrashing in Slider Drag Events

**Learning:**
Recalculating layout information via `getBoundingClientRect()` within a high-frequency event listener (like `mousemove` during slider drag) forces the browser to synchronously recalculate layout on every pixel moved, leading to layout thrashing and dropped frames.

**Action:**
Cache the output of `getBoundingClientRect()` during the initial `mousedown` event and pass it to the position-to-value conversion function instead of recalculating it on every `mousemove`.
