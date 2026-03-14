## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize Audio DSP Loop (Sample-outer vs Band-outer)

**Learning:**
Applying multiple DSP filters in series across an interleaved stereo buffer using a band-outer-loop approach causes poor CPU cache locality and forces redundant modulo arithmetic for channel index checking inside the hottest processing loop.

**Action:**
Switched to a sample-outer-loop approach using `chunks_exact_mut(CHANNELS)` to process stereo frames together. This eliminates `i % CHANNELS` arithmetic, maximizes CPU cache hits by accessing the audio buffer sequentially once, and pre-calculates active bands to avoid branching per sample.
