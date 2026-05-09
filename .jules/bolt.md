## 2026-05-09 - Audio Effect Loop Fusion
**Learning:** Sequential processing of independent DSP effects over the same buffer leads to multiple passes over the array, causing redundant memory reads/writes and potential cache misses.
**Action:** Apply loop fusion to process each sample entirely through all active effects in a single pass. Combine this with pre-computing effect parameters outside the loop, and use an early return if no effects are active to skip unnecessary iteration entirely.
