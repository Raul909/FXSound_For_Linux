## 2026-05-30 - Prevent O(N) DSP Loop Overhead on Flat EQ
**Learning:** In DSP loops, processing every sample is expensive. Even if inner loop logic is skipped, iterating over the buffer and re-assigning values (redundant memory writes) causes overhead.
**Action:** Use early returns (e.g., `if active_count == 0`) to completely skip O(N) loops when the effect state is inactive or flat.
