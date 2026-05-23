## 2026-05-23 - Loop Fusion in Audio DSP
**Learning:** When chaining multiple audio effects (Fidelity, Dynamic Compression, Bass Boost), iterating over the audio buffer sequentially for each effect causes unnecessary memory reads/writes and cache misses, multiplying the processing time.
**Action:** Use loop fusion to process each audio sample completely through all active effects in a single pass, and pre-calculate constant effect parameters outside the loop to minimize redundant floating-point math inside high-frequency DSP loops.
