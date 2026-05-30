## 2026-05-30 - Optimize DSP Effects via Loop Fusion
**Learning:** When chaining multiple DSP effects in a loop over audio buffers, processing each effect in its own loop causes redundant memory reads/writes and cache misses.
**Action:** Used loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculated constant parameters outside the loop to prevent redundant floating-point math.
