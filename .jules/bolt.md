## 2026-06-11 - [Early Return in Audio Engine]
**Learning:** In DSP loops, an early return can completely skip O(N) sample processing and prevent redundant memory writes when the effect state is flat or inactive.
**Action:** Use early returns (e.g., `if active_count == 0`) before expensive array iterations to optimize hot paths.
