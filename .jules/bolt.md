## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent Layout Thrashing in Slider Components

**Learning:**
React UI sliders (`EQBand` and `EffectSlider`) were calling `getBoundingClientRect()` inside their `mousemove` event handlers. This forced the browser to synchronously recalculate layout hundreds of times per second during mouse drag interactions, causing severe layout thrashing and high CPU usage.

**Action:**
Cache the output of `getBoundingClientRect()` during the initial `mousedown` event and reuse that cached `rect` during all subsequent `mousemove` calculations. The track dimensions do not change during a single drag gesture, eliminating the need to query the DOM repeatedly.
