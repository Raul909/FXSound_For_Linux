## 2025-05-14 - Optimize Audio Processing Loop and FFT

**Learning:**
The real-time audio loop was performing multiple vector allocations per iteration and re-planning the FFT on every call. In latency-sensitive threads like audio processing, heap allocations and expensive planning operations should be avoided.

**Action:**
Pre-allocated buffers in the audio loop and cached the FFT processor and complex buffers in the `AudioEngine`. Used in-place updates with `zip` and `chunks_exact_mut` to eliminate allocations.

## 2025-05-15 - Optimize audio EQ buffer iteration for cache locality

**Learning:**
Iterating over a large audio buffer multiple times for each active EQ band causes significant cache thrashing. Additionally, performing modulo arithmetic (`i % CHANNELS`) inside the hot loop to determine the stereo channel adds unnecessary overhead.

**Action:**
Reversed the loop ordering in `apply_eq` to iterate through the audio buffer once (sample-outer-loop). Used `.chunks_exact_mut(CHANNELS as usize)` to process stereo pairs simultaneously, eliminating the modulo operation. This maximizes cache locality, as each sample stays in a register while all active EQ filters are applied to it before moving to the next pair.
