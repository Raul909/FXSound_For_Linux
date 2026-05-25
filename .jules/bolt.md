## 2026-05-25 - DSP Loop Fusion
**Learning:** Sequential DSP effects mapped individually across an audio buffer can cause severe redundant memory reads/writes, leading to L1 cache misses and unnecessary recalculations.
**Action:** Apply loop fusion. Process each audio sample entirely through all active effects in a single pass and pre-calculate all floating-point effect coefficients outside the hot loop.
