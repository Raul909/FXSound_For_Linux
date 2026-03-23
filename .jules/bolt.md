## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-03-23 - Prevent Layout Thrashing in React Mouse Events
**Learning:**
Calculating layout properties like `getBoundingClientRect()` inside high-frequency event handlers such as `mousemove` triggers synchronous layout calculations (layout thrashing) on every emitted event, severely degrading UI drag performance.
**Action:**
Cache layout geometry on the initial `mousedown` event and pass the static rect object to helper functions used during the subsequent `mousemove` drag lifecycle to eliminate redundant layout recalcs.
