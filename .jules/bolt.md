## 2026-06-04 - Add early return to DSP EQ loop
**Learning:** DSP loops evaluating thousands of samples per second will still iterate over samples even when no filters are active, consuming unnecessary CPU cycles.
**Action:** Use an early return `if active_count == 0 { return; }` in tight audio processing loops to prevent redundant iterations and memory operations when the effect state is inactive.
