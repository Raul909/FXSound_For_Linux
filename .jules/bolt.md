## 2026-05-12 - Prevent redundant loop iterations in flat EQ state
**Learning:** In high-frequency DSP loops like apply_eq, even operations that do no semantic work (like iterating over an empty active filter slice and rewriting the same sample value back to memory) have a measurable O(N) CPU overhead per frame due to memory bandwidth.
**Action:** Always implement early returns (e.g., `if active_count == 0`) in audio processing pipelines to completely bypass O(N) sample processing loops when the effect state is flat/inactive.
