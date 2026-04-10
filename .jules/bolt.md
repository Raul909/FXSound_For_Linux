## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Layout Thrashing in Slider Drag Events

**Learning:**
Calling `getBoundingClientRect()` inside a high-frequency `mousemove` event handler recalculates layout metrics on every single mouse movement pixel, leading to severe layout thrashing (synchronous layout forces) and jittery, CPU-intensive drag performance.

**Action:**
Calculate and cache the result of `getBoundingClientRect()` once during the initial `mousedown` event, and reuse that cached bounding rect for all subsequent calculations within the nested `mousemove` closure.
