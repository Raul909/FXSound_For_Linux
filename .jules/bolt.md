## 2026-05-16 - Prevent Redundant DSP Memory Operations
**Learning:** In DSP loops like apply_eq, even if the inner loop over filters is empty (e.g., active_count == 0), the outer loop over all audio samples will still execute. This causes redundant O(N) memory reads and writes (*sample = s) which wastes memory bandwidth and CPU cycles when the effect state is flat.
**Action:** Always implement early returns (e.g., if active_count == 0) in DSP functions to completely skip the O(N) processing loop when the effect is inactive.
