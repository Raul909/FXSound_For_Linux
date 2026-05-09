## 2026-05-09 - Loop Fusion in DSP Chains
**Learning:** Chaining multiple independent DSP effects via separate buffer iterations causes redundant memory reads/writes and cache misses.
**Action:** Use loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculate invariant parameters outside the loop.
