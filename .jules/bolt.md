## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize Audio DSP Filter Pipelines
**Learning:**
When optimizing Rust audio DSP pipelines that apply multiple filters (like a 10-band EQ), a "band-outer" loop combined with sample-level modulo arithmetic (`i % CHANNELS`) is inefficient. Modulo arithmetic is slow, and looping over bands first means iterating through the entire audio buffer 10 times, destroying CPU cache locality.
**Action:**
Switched to a "sample-outer" approach using `.chunks_exact_mut(CHANNELS)` to process complete stereo frames. This approach eliminates modulo arithmetic, maximizes cache locality, and ensures the buffer is only traversed once. Pre-computing active bands outside the hot loop further avoided branching and heap allocations.
