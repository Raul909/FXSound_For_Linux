## 2024-06-13 - [Early return from `apply_eq` when flat]
**Learning:** In DSP loops (e.g., `apply_eq` in `src-tauri/src/audio.rs`), it calculates the active bands, but does not use an early return to skip expensive processing entirely when no bands are active.
**Action:** Use early returns (`if active_count == 0`) to skip expensive O(N) audio sample processing loops and prevent redundant memory writes entirely when the effect state is flat/inactive.
## 2026-06-13 - [Early return from apply_eq when flat]
**Learning:** In DSP loops (e.g., apply_eq in src-tauri/src/audio.rs), it calculates the active bands, but does not use an early return to skip expensive processing entirely when no bands are active.
**Action:** Use early returns (if active_count == 0) to skip expensive O(N) audio sample processing loops and prevent redundant memory writes entirely when the effect state is flat/inactive.
