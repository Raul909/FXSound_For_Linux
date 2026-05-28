## 2026-05-28 - Early Return in DSP Loops
**Learning:** In high-frequency DSP loops like audio processing, a flat/inactive effect state can still result in O(N) redundant iterations and memory writes if not explicitly checked.
**Action:** Use early returns (e.g., `if active_count == 0`) before expensive O(N) sample processing loops to skip them entirely and prevent redundant operations when no processing is required.
