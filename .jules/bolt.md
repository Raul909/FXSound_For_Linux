## 2026-05-10 - Loop Fusion in DSP Processing
**Learning:** Chaining DSP effects using consecutive loops over the audio buffer causes redundant memory reads/writes and cache misses.
**Action:** Employ loop inversion/fusion in DSP paths to process each audio sample entirely through all active effects in a single pass, and pre-calculate constant parameters outside the loop to minimize floating-point overhead.
