
## 2026-05-07 - Loop Fusion in DSP Chains
**Learning:** Chaining DSP effects as separate iterations over the audio buffer causes redundant memory reads/writes and cache overhead.
**Action:** Apply loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculate constant parameters outside the loop.
