## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Prevent layout thrashing in high-frequency event listeners

**Learning:**
Recalculating DOM geometry via `getBoundingClientRect()` inside a high-frequency event listener like `mousemove` causes forced synchronous layout (layout thrashing), which severely degrades drag performance and causes UI stuttering.

**Action:**
In React components that implement custom drag interactions (like `EffectSlider` and `EQBand`), cache the output of `getBoundingClientRect()` once during the initial `mousedown` event. Pass this cached geometry to the position-to-value conversion functions, avoiding costly DOM recalculations on every pixel moved.
