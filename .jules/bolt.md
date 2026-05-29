## 2026-05-29 - Prevent O(N) Iteration on Flat Audio DSP State
**Learning:** In high-frequency audio DSP loops, even an empty inner loop (e.g., iterating when no EQ bands are active) causes an expensive O(N) pass with redundant memory writes over the audio buffer.
**Action:** Always add an explicit early return (e.g., `if active_count == 0`) before buffer iteration loops when the effect state is inactive to avoid unnecessary CPU cycles and cache invalidation.
