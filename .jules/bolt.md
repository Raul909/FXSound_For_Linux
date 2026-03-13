## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize DSP Audio Processing Loops via Sample-Outer Approach

**Learning:**
When applying multiple biquad filters across an array of samples, using a band-outer loop (iterating through all samples for each EQ band) results in poorer CPU cache locality than a sample-outer loop (iterating through all EQ bands for each sample/frame). Furthermore, calculating the channel index inside the loop using modulo arithmetic (`i % CHANNELS`) is computationally expensive and unnecessary when audio channels are interleaved.

**Action:**
Restructured the EQ processing loop in `src-tauri/src/audio.rs` to process stereo frames using `chunks_exact_mut(CHANNELS)`. This eliminates the modulo calculation and improves cache hit rates since each sample block is fully processed by all active filters before moving to the next block.
