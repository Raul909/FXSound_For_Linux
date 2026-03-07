## 2025-03-07 - Cache FFT planner and pre-allocate buffers
**Learning:** `rustfft`'s `FftPlanner` shouldn't be repeatedly invoked in hot audio loops, and pre-allocating scratch/complex buffers prevents significant heap allocation overhead per tick.
**Action:** Always cache the `FftPlanner` when doing repeated FFT computations, especially in real-time environments where allocations have a large impact. Pre-allocate all buffers outside the loop.
