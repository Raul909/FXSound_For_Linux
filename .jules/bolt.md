## 2026-05-22 - DSP Effects Loop Fusion
**Learning:** In audio DSP processing, applying multiple effects sequentially by iterating over the entire sample buffer repeatedly causes unnecessary memory reads/writes and CPU cache misses.
**Action:** Employ loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculate constant parameters outside the loop to reduce overhead.
