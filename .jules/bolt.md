
## 2026-05-07 - Loop Fusion for DSP Effect Chaining
**Learning:** In DSP loops like `apply_effects` within `src-tauri/src/audio.rs`, iterating over the entire buffer for each effect causes redundant memory reads/writes and cache misses.
**Action:** Always pre-calculate constant parameters outside the loop and employ loop fusion to process each sample entirely through all active effects in a single pass to improve CPU cache locality.
