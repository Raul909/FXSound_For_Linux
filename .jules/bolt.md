## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2026-04-29 - Inefficient Loop Nesting in EQ Processing
**Learning:** Swapping nested loops in DSP paths to iterate over the buffer once and apply all active filters per sample improves CPU cache locality and reduces memory bandwidth overhead.
**Action:** Refactored `AudioEngine::apply_eq` to pre-compute active EQ bands and process each audio sample completely through all active filters before moving to the next sample.
