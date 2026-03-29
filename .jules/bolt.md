## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Eliminate Modulo Arithmetic in Audio Loop

**Learning:**
In the Rust audio DSP pipeline applying multiple filters, the inner loop was using a sample-outer-loop approach that included modulo arithmetic (`i % CHANNELS`). Modulo division is notoriously slow and should be avoided in latency-sensitive, hot real-time audio threads.

**Action:**
Replaced the `enumerate` and modulo operator with block-processing by using `output.chunks_mut(CHANNELS as usize)` in the outer loop, and then iterating over the channels within the chunk. This eliminates the expensive modulo operation and keeps the fast path clean.
