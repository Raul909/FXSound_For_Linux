## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize Slider Event Handlers (Prevent Layout Thrashing)

**Learning:**
In custom UI elements like sliders (`EQBand`, `EffectSlider`), recalculating `getBoundingClientRect()` on every `mousemove` event triggers synchronous layout calculations. This causes layout thrashing and degrades performance, particularly on high refresh rate displays or slower devices.

**Action:**
Cache the layout values (bounding rects) during the initial `mousedown` event and reuse them in the `mousemove` handlers. This eliminates redundant layout calculations because the slider's physical dimensions shouldn't change while the user is actively dragging.
