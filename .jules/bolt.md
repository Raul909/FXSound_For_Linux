## 2026-06-10 - Loop Fusion in DSP Processing
**Learning:** In audio DSP loops, applying sequential effects in isolated passes causes redundant O(N) iterations, memory reads, and memory writes over the audio buffer, increasing CPU cache thrashing.
**Action:** Used loop fusion in `apply_effects` to process each sample entirely through all active effects in a single pass, and pre-calculated effect constants outside the loop to avoid redundant floating-point math. Added early returns to skip processing entirely when all effects are inactive.
