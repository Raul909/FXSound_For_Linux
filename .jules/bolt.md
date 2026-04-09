## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2024-04-09 - Avoid layout thrashing in high-frequency React event listeners
**Learning:** Calling `getBoundingClientRect()` within a high-frequency event listener like `mousemove` causes forced synchronous layout calculations (layout thrashing) on every pixel moved, significantly degrading performance, especially in components like UI sliders (e.g., `EQBand`, `EffectSlider`).
**Action:** Cache the result of `getBoundingClientRect()` during the initial `mousedown` event and pass the cached bounding rect to the coordinate calculation functions used during `mousemove`.
