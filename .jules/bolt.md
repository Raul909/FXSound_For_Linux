
## 2026-05-26 - Skip flat DSP loops
**Learning:** In DSP loops (like audio sample processing), if the effect state is inactive/flat, looping over the entire buffer even when doing nothing causes redundant memory writes, cache misses, and wastes CPU cycles.
**Action:** Use early returns (e.g., `if active_count == 0 { return; }`) to skip O(N) operations entirely when the configuration requires no work.
