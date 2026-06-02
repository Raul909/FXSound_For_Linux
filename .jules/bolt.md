## 2026-06-02 - Optimize Audio EQ Loop

**Learning:** In DSP loops, processing arrays sample-by-sample without checking if the effect is active causes unnecessary memory iteration and redundant array writes.
**Action:** Always check for early returns (e.g., `if active_count == 0`) before executing O(N) audio sample processing loops to skip redundant computations entirely when the effect state is flat or inactive.
