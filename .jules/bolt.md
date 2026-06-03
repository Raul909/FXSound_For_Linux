## 2026-06-03 - Add early return to DSP loop in apply_eq
**Learning:** In DSP loops like `apply_eq`, processing arrays of samples per frame when effects are inactive creates significant redundant CPU and memory overhead.
**Action:** Always implement early returns (e.g., `if active_count == 0`) before expensive O(N) sample processing loops when the effect state is flat or inactive to skip processing and prevent redundant memory writes entirely.
