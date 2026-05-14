## 2026-05-14 - Prevent O(N) memory writes in inactive DSP loop
**Learning:** Even when skipping processing for inactive EQ bands, the outer sample iteration loop still performs O(N) redundant memory reads and writes (`*sample = s`) which consumes memory bandwidth and CPU cycles unnecessarily.
**Action:** Always add early returns (`if active_count == 0 { return; }`) in DSP processing pipelines to completely bypass buffer iterations when the effect or EQ state is flat/inactive.
