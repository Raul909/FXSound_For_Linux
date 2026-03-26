## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Eliminate Modulo Arithmetic in Audio Loop

**Learning:**
The inner processing loop for EQ bands utilized modulo arithmetic (`i % CHANNELS`) for every sample, which can be computationally expensive and prevent optimal compiler vectorization/unrolling, especially in hot loops where performance is critical.

**Action:**
Replaced the manual channel index calculation with block processing. Modified the loop to use `output.chunks_mut(CHANNELS as usize)` and iterated over each chunk, allowing the inner loop to use simple array indices (`channel`). This avoids the expensive modulo operation and better leverages memory localities and compiler optimizations.
