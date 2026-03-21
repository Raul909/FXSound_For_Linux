## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.
## 2025-05-14 - Optimize EQ Inner Loop
**Learning:**
In real-time audio loops, avoid slow modulo arithmetic (`i % CHANNELS`). Modulo inside a hot loop adds significant overhead.
**Action:**
Instead of `enumerate` and modulo, iterate over chunks using `chunks_mut(CHANNELS)`. This eliminates the expensive modulo operator and enables processing multiple channels explicitly in the inner loop.
