## 2026-05-23 - Loop Fusion in DSP pipelines
**Learning:** In audio DSP processing functions like `apply_effects`, iterating over the audio buffer repeatedly for each individual effect degrades CPU cache locality and wastes memory bandwidth.
**Action:** Apply loop fusion to combine multiple effect passes into a single iteration over the buffer, and pre-calculate any constant parameters outside the loop.
