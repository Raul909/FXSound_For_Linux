## 2026-05-13 - Early return in apply_eq
**Learning:** In DSP loops like `apply_eq`, processing arrays linearly can incur an expensive overhead even when no filters are active. Using an early return prevents O(N) redundant memory writes and iteration.
**Action:** Always check array lengths/active counts before entering expensive O(N) loops over DSP audio buffers.
