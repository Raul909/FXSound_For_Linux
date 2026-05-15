
## 2026-05-15 - Early returns in DSP loops
**Learning:** DSP loops that iterate over audio buffers can perform redundant memory reads/writes and computations even when the effect state is flat/inactive.
**Action:** Use early returns (e.g., `if active_count == 0`) to skip expensive O(N) audio sample processing entirely and prevent unnecessary performance overhead.
