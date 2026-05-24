## 2026-05-24 - Early Return for Inactive Filters
**Learning:** In stateful DSP loops like `apply_eq` in `src-tauri/src/audio.rs`, processing chunks sample-by-sample through flat (inactive) effect filters causes redundant memory reads/writes and iterations. We can skip expensive loops entirely if all effect counts drop to zero.
**Action:** Implement an early return in DSP loops (`if active_count == 0`) to prevent redundant loop execution when the effect state is flat.
