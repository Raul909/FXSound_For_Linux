## 2026-05-08 - Optimize effects processing via loop fusion

**Learning:** When chaining multiple DSP effects (fidelity, dynamic compression, bass boost) in Rust, iterating over the sample buffer for each effect causes redundant memory reads/writes and cache misses.

**Action:** Employed loop fusion to process each sample entirely through all active effects in a single pass. Also, constant parameters derived from effect intensities were pre-calculated outside the loop to prevent redundant floating-point math inside the inner sample processing loop.
