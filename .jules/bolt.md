## 2026-05-24 - DSP Loop Early Returns
**Learning:** In high-frequency DSP loops (like audio sample processing running at 48kHz), empty inner loops still incur significant overhead from iterating over large arrays (O(N) memory reads/writes).
**Action:** Always implement early returns when processing states (like EQ or effects) are flat/inactive to skip the array iteration entirely and save CPU cycles.
