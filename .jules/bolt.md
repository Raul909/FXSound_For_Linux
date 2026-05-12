## 2026-05-12 - Loop Fusion in DSP Processing
**Learning:** When chaining multiple DSP effects, looping over the audio buffer multiple times causes redundant memory reads/writes and cache misses. Floating-point constants inside effect loops also cause redundant math operations.
**Action:** Employ loop fusion to process each sample completely through all active effects in a single pass, and pre-calculate parameter constants outside the loop.
