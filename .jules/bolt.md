## 2026-05-27 - Skip expensive DSP loops when inactive
**Learning:** In high-frequency DSP code, iterating over an array to process data is expensive even if the inner loop does nothing. A flat/inactive effect can still trigger redundant array element reads and writes.
**Action:** Always add early returns in sample processing functions (like apply_eq) if the computed active filter count or effect state is zero, bypassing the O(N) sample loop entirely.
