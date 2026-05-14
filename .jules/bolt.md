## 2026-05-14 - Loop Fusion in DSP Effects
**Learning:** Multiple passes over audio buffers for chaining effects cause redundant memory reads/writes and cache misses. Pre-computing loop invariants and fusing the effect iterations into a single sample-by-sample pass significantly reduces memory bandwidth overhead in DSP loops.
**Action:** Apply loop inversion/fusion to process each audio sample completely through all active effects before moving to the next sample, and hoist constants out of the loop.
