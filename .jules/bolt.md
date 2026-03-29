## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize DSP Inner Loop Modulo Arithmetic
**Learning:**
Modulo arithmetic (`i % CHANNELS`) is surprisingly expensive in the innermost loop of real-time DSP pipelines. The `apply_eq` function was running this for every sample across 10 EQ bands.
**Action:**
Use block-processing approaches (like `.chunks_mut(CHANNELS)`) to handle multi-channel interleaved buffers, completely eliminating modulo division operations from the hot path.
