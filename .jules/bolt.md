## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Prevent layout thrashing in slider drag events

**Learning:**
Calling `getBoundingClientRect()` inside high-frequency event listeners like `mousemove` causes forced synchronous layout (layout thrashing), which severely degrades drag performance and causes UI stuttering.

**Action:**
Cache the output of `getBoundingClientRect()` during the initial `mousedown` event and pass the cached rectangle to the computation function, instead of recalculating it on every pixel moved during `mousemove`.
