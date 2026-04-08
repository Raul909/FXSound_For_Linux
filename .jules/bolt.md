## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2025-04-08 - Preventing Layout Thrashing in Slider Components
**Learning:** Calling `getBoundingClientRect()` inside high-frequency event listeners (like `mousemove`) forces the browser to recalculate layout synchronously on every event firing, causing severe layout thrashing and performance degradation during interactions.
**Action:** Always cache the output of `getBoundingClientRect()` during the initial interaction event (e.g., `mousedown`) and pass it to the handler calculating the value during subsequent high-frequency events (e.g., `mousemove`), rather than recalculating it dynamically.
