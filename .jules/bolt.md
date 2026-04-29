## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2026-04-29 - Optimize Audio Processing Loop Cache Locality

**Learning:**
Iterating through the entire audio buffer for each active EQ band in sequence causes excessive memory traffic and poor cache locality, especially in real-time audio processing loops.

**Action:**
Inverted the processing loop in `apply_eq` to iterate over samples in the outer loop and active filters in the inner loop, pre-identifying active bands in a stack-allocated array to avoid heap allocations and branching in the hot path.
