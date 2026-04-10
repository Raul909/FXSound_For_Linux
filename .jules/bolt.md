## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Layout Thrashing in Slider Drag Events

**Learning:**
Calling `getBoundingClientRect()` inside a high-frequency event listener like `mousemove` forces synchronous layout calculation, which causes layout thrashing and drops frame rates during slider drag interactions.

**Action:**
Extract `getBoundingClientRect()` out of the `mousemove` event handler and calculate it only once during the initial `mousedown` event, passing the cached `rect` into the coordinate mapping functions.
