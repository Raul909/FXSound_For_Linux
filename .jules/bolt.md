## 2026-05-19 - Early return in DSP loops
**Learning:** O(N) audio sample processing loops over a buffer can consume CPU and memory bandwidth even if the inner loop does no math, because of the outer loop iteration overhead and potential memory accesses.
**Action:** Always use early returns (e.g., `if active_count == 0`) before an expensive DSP loop if we know the effect state is entirely flat or inactive.
