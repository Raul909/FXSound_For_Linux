## 2026-05-04 - DSP Loop Early Return
**Learning:** In audio DSP pipelines (like `apply_eq`), skipping the sample processing loop entirely when parameters are neutral (flat EQ) avoids unnecessary O(N) buffer iteration, significantly reducing CPU usage during idle/flat states.
**Action:** Always check if a DSP processing block can be bypassed entirely using an early return before entering per-sample loops.
