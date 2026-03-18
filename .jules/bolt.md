## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Layout Thrashing in Slider Components

**Learning:**
Calling `getBoundingClientRect()` inside a high-frequency `mousemove` event listener causes synchronous layout recalculations (layout thrashing) on every pixel moved, leading to UI stuttering during interactions like dragging sliders.

**Action:**
Cache the `getBoundingClientRect()` output once on the `mousedown` event and pass the cached bounding box into calculations during `mousemove` events to avoid forcing synchronous reflows.
