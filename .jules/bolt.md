## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Block-Based Interleaved Stereo Processing
**Learning:** In audio DSP pipelines with interleaved stereo buffers, using `.enumerate()` with modulo arithmetic (`i % CHANNELS`) to determine the channel creates slow branching/division inside the hot loop. Furthermore, interleaving samples across channels can cause filter coefficients to be evicted from fast CPU registers.
**Action:** Replaced modulo-based iteration with `.chunks_mut(CHANNELS)` to process channels directly. This avoids modulo operations and keeps filter states/coefficients resident in CPU registers, significantly improving throughput for multi-band EQs.
