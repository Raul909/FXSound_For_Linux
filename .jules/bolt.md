## 2026-05-13 - Prevent redundant iteration in flat DSP loops
**Learning:** Even when the inner loops over DSP filters are bypassed (e.g., when no EQ bands are active), iterating over the entire audio buffer to read and write each sample redundantly causes unnecessary memory bandwidth and CPU overhead.
**Action:** Add early returns (e.g., `if active_count == 0`) before the per-sample processing loop to entirely skip the O(N) iteration when the effect state is inactive.
