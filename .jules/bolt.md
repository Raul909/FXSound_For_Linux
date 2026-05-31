
## 2026-05-31 - O(N) Loop Prevention via Early Returns in DSP
**Learning:** Even when inner loops over active DSP parameters (like EQ bands) have zero iterations, the outer loop over the audio buffer (O(N)) still executes, causing redundant reads and writes (e.g., `*sample = s;`).
**Action:** Add explicit early returns (`if active_count == 0 { return; }`) before the O(N) sample processing loops when the effect state is flat/inactive to completely bypass the O(N) iteration overhead.
