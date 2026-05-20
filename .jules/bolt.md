## 2026-05-20 - Early return in DSP loops
**Learning:** In audio processing loops, when an effect like EQ is completely flat/inactive, allowing the outer sample processing loop to still execute over the entire buffer causes unnecessary redundant iteration and memory writes (O(N) overhead).
**Action:** Always implement early return guards (e.g., `if active_count == 0 { return; }`) in DSP functions to entirely skip buffer iteration when the effect state is inactive.
