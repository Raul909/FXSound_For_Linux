## 2026-06-08 - DSP Early Return Optimization
**Learning:** In audio DSP processing loops like `apply_eq`, even when no active effects or filters are present, looping over each audio sample incurs O(N) loop iteration overhead and redundant read/write operations per sample.
**Action:** Always check if the processing state is inactive (e.g., `active_count == 0`) and use an early return to entirely bypass the per-sample loop, preventing unnecessary iterations and memory writes.
