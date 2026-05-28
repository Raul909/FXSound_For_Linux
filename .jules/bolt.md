## 2026-05-28 - DSP Loop Bypass Optimization
**Learning:** In audio processing, even simple parameter checks can be costly if executed per-sample. Skipping expensive O(N) processing loops entirely when effect states are flat prevents redundant memory reads/writes.
**Action:** Implemented an early return in the `apply_eq` function of `src-tauri/src/audio.rs` when `active_count == 0`.
