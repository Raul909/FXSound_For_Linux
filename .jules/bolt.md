## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Cache DOM rects during custom slider drag to prevent layout thrashing
**Learning:**
Recalculating `getBoundingClientRect()` inside a high-frequency `mousemove` event handler causes significant layout thrashing. In custom React UI sliders (`EQBand` and `EffectSlider`), this caused unnecessary DOM recalculations on every single pixel moved.
**Action:**
Calculate and cache the slider track's bounding rect precisely once during the initial `mousedown` event, and reference that cached `rect` inside the nested `mousemove` handler to dramatically improve drag performance.
