## 2026-05-18 - Audio DSP Loop Fusion
**Learning:** Iterating over the audio buffer multiple times for different effects (Fidelity, Dynamic Compression, Bass Boost) causes redundant memory reads/writes and cache misses.
**Action:** Use loop fusion to combine multiple effect passes into a single pass over the buffer. Pre-calculate constant parameters (like thresholds and ratios) outside the loop to minimize repeated floating-point math inside the high-frequency DSP loop.
