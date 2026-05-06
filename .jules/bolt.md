
## 2026-05-06 - Loop Fusion in DSP Effects
**Learning:** Sequential audio effects loops (e.g., fidelity, dynamic compression, bass boost) iterating over the same buffer independently cause redundant memory reads/writes and cache misses, similar to unoptimized EQ loops. Pre-calculating variables outside the loop also prevents redundant floating-point math per sample.
**Action:** When chaining DSP effects, use loop fusion to process each sample entirely through all active effects in a single pass, and pre-calculate any constant parameter math outside the loop.
