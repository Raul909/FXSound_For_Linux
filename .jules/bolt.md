## 2026-05-16 - DSP Loop Fusion
**Learning:** Chaining DSP effects with separate iterations per effect reduces CPU cache locality and increases memory bandwidth overhead.
**Action:** Use loop fusion to process each audio sample entirely through all active effects in a single pass, and pre-calculate constant parameters outside the loop to prevent redundant floating-point math and cache misses.
