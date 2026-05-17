## 2026-05-17 - DSP Loop Fusion
**Learning:** Chaining DSP effects individually causes redundant iterations over the audio buffer, leading to CPU cache misses and unnecessary memory bandwidth overhead.
**Action:** Use loop fusion to process each audio sample completely through all active effects in a single pass, and pre-calculate invariant parameters outside the loop.
