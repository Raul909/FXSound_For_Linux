## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-29 - Improve Audio Processing Loop Cache Locality

**Learning:**
Iterating over the entire audio buffer for each active EQ band results in poor cache locality and redundant memory accesses. By reversing the loop hierarchy—processing all active EQ bands for a single sample before moving to the next sample—we ensure the sample stays in the CPU cache, minimizing memory operations in the hot path.

**Action:**
Pre-identify active EQ bands into a small stack-allocated array (e.g., `[usize; 10]`) before the sample loop. Then, iterate through the buffer once, applying all active filters to each sample.
