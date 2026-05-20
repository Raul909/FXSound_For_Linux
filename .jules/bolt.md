
## 2026-05-20 - Skip Unnecessary DSP Loops
**Learning:** In audio processing pipelines, iterating over an output buffer when an effect is effectively inactive (e.g., flat EQ) still costs O(N) operations due to memory read/writes and array indexing overhead, even if the inner processing logic is empty.
**Action:** Implement early returns in DSP functions when effect states are inactive to entirely skip expensive sample processing loops.
