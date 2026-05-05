## 2026-05-05 - DSP Loop Early Return
**Learning:** DSP loops operating on large audio buffers can incur unnecessary O(N) overhead (even just iterating and doing modulo arithmetic) when the effect is bypassed (e.g., all EQ bands flat).
**Action:** Always add early returns (`if active_count == 0 { return; }`) in DSP pipelines to skip the expensive per-sample iteration when processing is dynamically inactive.
