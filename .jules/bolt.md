## 2026-06-09 - Prevent Redundant Writes in Inactive DSP Loops
**Learning:** Even when inner processing loops are empty (e.g., 0 active EQ bands), iterating over the main buffer and reassigning the sample to itself triggers unnecessary memory writes across the entire audio chunk.
**Action:** Always implement early returns (if active_count == 0) in DSP chains to completely skip the O(N) sample loop when the effect state is flat/inactive.
