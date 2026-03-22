## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-14 - Optimize EQ Processing Loop Modulo Arithmetic

**Learning:**
In the real-time audio loop (`apply_eq`), channel separation for stereo processing was done using an expensive modulo operation `let channel = i % (CHANNELS as usize)` inside the hot inner sample loop. This is a common performance bottleneck in audio DSP.

**Action:**
Switched to a chunk-based block-processing approach by using `.chunks_mut(CHANNELS as usize)` on the output buffer and then iterating through the inner frame with `enumerate()`. This eliminates the need for modulo arithmetic inside the sample iteration block, reducing CPU cycles per frame.
