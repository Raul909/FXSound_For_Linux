## 2026-05-25 - DSP Effects Loop Fusion
**Learning:** Chaining multiple audio effects using separate loops creates redundant memory reads and writes, increasing cache misses in high-frequency DSP paths.
**Action:** Use loop fusion to combine multiple effect stages into a single pass over the buffer, and pre-calculate constants outside the loop to minimize redundant floating-point math per sample.
