## 2026-06-14 - Early Returns in DSP Loops
**Learning:** In DSP loops (e.g., apply_eq in src-tauri/src/audio.rs), if the effect state is flat/inactive, iterating over the buffer to write identical values back to memory causes redundant O(N) memory writes.
**Action:** Use early returns (e.g., if active_count == 0) to skip the expensive O(N) audio sample processing loop and prevent redundant memory writes entirely.
