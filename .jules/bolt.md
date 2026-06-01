## 2026-06-01 - Loop Fusion for Chained DSP Effects
**Learning:** In audio processing, chaining sequential effects by iterating over the buffer multiple times causes redundant memory reads/writes and cache misses.
**Action:** Used loop fusion in `apply_effects` to process each sample through all active effects in a single pass, and pre-calculated constant parameters outside the loop to avoid redundant floating-point math.
