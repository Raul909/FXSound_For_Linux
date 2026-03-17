## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize Audio DSP Loop Channel Indexing Modulo

**Learning:**
Modulo division (`% CHANNELS`) inside hot DSP loops over audio sample arrays is relatively expensive and slows down stream processing.

**Action:**
Replaced the `modulo` operation with `.chunks_mut(CHANNELS as usize)` iterators, allowing native block-based channel addressing with `.enumerate()`. This speeds up `apply_eq` by leveraging chunk boundaries to maintain channel state rather than per-sample division.
