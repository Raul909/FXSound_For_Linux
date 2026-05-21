## 2026-05-21 - Early returns in DSP loops prevent redundant O(N) memory writes
**Learning:** Even if an inner processing loop is skipped (e.g. iterating an empty slice of active filters), the outer loop iterating over all audio samples still incurs expensive memory reads and writes, wasting memory bandwidth and CPU cycles when the state is flat.
**Action:** Always add early returns (e.g., `if active_count == 0 { return; }`) before iterating over large audio/DSP buffers if the effect state is inactive or flat.
