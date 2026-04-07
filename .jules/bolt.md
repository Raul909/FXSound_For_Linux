## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Layout Thrashing in Custom Sliders
**Learning:** Calling `getBoundingClientRect()` inside a high-frequency `mousemove` event handler causes synchronous layout recalculations (layout thrashing), leading to janky slider performance.
**Action:** Always cache the results of DOM layout queries like `getBoundingClientRect()` during the initial `mousedown` event and reuse them in the `mousemove` handlers when implementing custom drag interactions.
