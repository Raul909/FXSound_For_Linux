## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-17 - Prevent Layout Thrashing in Slider Mouse Events
**Learning:** Calling getBoundingClientRect() on every mousemove event inside React components causes continuous synchronous layout thrashing, severely degrading performance during drag interactions.
**Action:** Cache the result of getBoundingClientRect() during the initial mousedown event and pass it to the movement calculations, completely avoiding DOM reflows during the high-frequency mousemove events.
