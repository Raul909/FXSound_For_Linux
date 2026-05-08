## 2026-05-08 - Loop Fusion in DSP Effects
**Learning:** Processing multiple DSP effects in separate loops over the same audio buffer causes redundant memory reads/writes and cache thrashing. Re-evaluating constants inside the loop further degrades performance.
**Action:** When chaining DSP effects, use loop fusion to process each sample entirely through all active effects in a single pass, and pre-calculate parameter constants outside the loop.
