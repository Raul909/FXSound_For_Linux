## 2026-05-18 - Loop Fusion in DSP Effect Chains
**Learning:** Chaining DSP effects sequentially with individual loops causes redundant memory reads/writes and potential cache misses.
**Action:** Apply loop fusion to process each sample entirely through all active effects in a single pass, and pre-calculate constant parameters outside the loop to reduce floating-point math overhead in high-frequency audio paths.
