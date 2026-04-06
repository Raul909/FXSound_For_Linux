## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2024-05-18 - Caching getBoundingClientRect in Event Listeners
**Learning:** Calling `getBoundingClientRect()` inside high-frequency event listeners like `mousemove` causes severe layout thrashing (forced synchronous layout), significantly degrading UI responsiveness, especially when tracking slider drags.
**Action:** Always cache the output of `getBoundingClientRect()` during the initial interaction event (e.g., `mousedown`) and reuse the cached bounding box in the subsequent high-frequency event handlers (`mousemove`), as the element's position on screen rarely changes during a single continuous drag interaction.
