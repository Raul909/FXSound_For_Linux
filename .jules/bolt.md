
## 2026-05-17 - Loop Fusion in DSP Effects
**Learning:** When chaining DSP effects, sequentially looping over the entire audio buffer for each effect causes redundant memory reads/writes and cache misses.
**Action:** Apply loop fusion to process each audio sample completely through all active effects in a single pass, and pre-calculate constant parameters outside the loop to reduce overhead.
