## 2026-05-21 - Early Return in Flat DSP Loops
**Learning:** Even when DSP effect loops process zero active bands, iterating over the entire audio buffer and re-assigning values causes unnecessary O(N) memory writes and CPU cycles.
**Action:** Always add early returns (e.g., `if active_count == 0 { return; }`) before entering expensive sample-processing loops when the effect state is completely flat or inactive.
