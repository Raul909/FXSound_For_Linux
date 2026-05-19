## 2026-05-19 - Loop fusion in DSP effects
**Learning:** In DSP audio processing loops, chaining effects by iterating over the same audio buffer multiple times causes redundant memory reads/writes and cache misses.
**Action:** Apply loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculate all constant effect parameters outside the loop.
