## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - DSP Loop Cache Locality

**Learning:**
Iterating over stereo frames `output.chunks_exact_mut(CHANNELS)` in the outer loop instead of processing each frequency band across the entire buffer maximizes CPU cache locality and eliminates expensive per-sample modulo arithmetic for determining channel numbers. Real-time audio threads should also pre-allocate any required state buffers to avoid Vec allocations (`[0usize; 10]`).

**Action:**
Use a sample-outer-loop/frame-outer-loop instead of a band-outer-loop for apply-in-series DSP pipelines. Use fixed-size stack arrays to avoid allocations.
