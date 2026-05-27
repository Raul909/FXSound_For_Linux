## 2026-05-27 - DSP Loop Fusion
**Learning:** Sequential DSP effects mapped individually across an entire audio buffer cause redundant memory reads, writes, and potential cache misses.
**Action:** Use loop fusion to combine multiple effect passes into a single loop, and pre-calculate any constant effect parameters outside the hot loop.
