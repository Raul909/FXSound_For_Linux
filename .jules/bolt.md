## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Prevent Layout Thrashing in Slider Event Handlers

**Learning:**
React event handlers for high-frequency events like `mousemove` (used for custom sliders) can cause severe performance degradation (layout thrashing) if they recalculate layout geometry on every pixel moved. The `getBoundingClientRect()` method forces the browser to synchronously calculate layout, which is highly inefficient when called repeatedly inside a `mousemove` loop.

**Action:**
Cache the layout output (the `rect` from `getBoundingClientRect()`) during the initial `mousedown` event instead of calculating it inside the conversion functions (`xToValue`, `yToGain`) triggered by every `mousemove` event. This avoids repeated layout recalculations while the user drags the slider.
