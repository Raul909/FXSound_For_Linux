## 2024-03-08 - Optimize FFT Processing in Audio Loop
**Learning:** Re-planning an FFT (`FftPlanner::new()`) and performing heap allocations (`Vec::collect()`) on every single frame inside a real-time audio processing loop is a massive performance bottleneck.
**Action:** Always pre-plan FFT processors and pre-allocate complex/scratch buffers within the engine initialization. Update magnitude data in-place to avoid dynamic heap allocations during high-frequency callbacks.
