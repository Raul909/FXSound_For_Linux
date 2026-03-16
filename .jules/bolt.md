## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimizing Biquad Filter Loop

**Learning:**
In DSP pipelines where multiple filters (like a 10-band EQ) are applied to an audio buffer, looping over bands in the outer loop and samples in the inner loop can cause poor CPU cache locality and necessitates slow modulo arithmetic (`i % CHANNELS`).

**Action:**
Changed the algorithm to a block-processing approach (band-outer-loop, sample-inner-loop) using `.chunks_mut(CHANNELS)`. Block processing keeps a single filter's coefficients loaded in fast CPU registers during the inner loop instead of repeatedly swapping states per-sample (which causes register spilling). Using `.chunks_mut(CHANNELS)` eliminates the slow modulo arithmetic (`i % CHANNELS`) while still safely supporting any number of channels without hardcoding `[0]` or `[1]`. Tracked active bands with a stack-based array to skip inactive bands without heap allocations.
