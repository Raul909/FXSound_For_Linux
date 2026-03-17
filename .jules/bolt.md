## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize audio DSP block processing
**Learning:** In audio DSP pipelines applying multiple filters, standard sample-outer-loop/sample-by-sample processing with modulo operations (`i % CHANNELS`) is inefficient and can cause register spilling.
**Action:** Use a block-processing approach with `chunks_mut` or `chunks_exact_mut` (e.g., `chunks_exact_mut(CHANNELS as usize)`) in the inner loop. This keeps filter coefficients loaded in fast CPU registers and eliminates slow modulo arithmetic.
