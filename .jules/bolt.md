## 2026-05-03 - [DSP Loop Early Return]
**Learning:** Even when inner loops are skipped due to empty collections, the outer iteration over large buffers still incurs overhead from iterator progression, math, and variable assignment.
**Action:** Always check if a stateful condition (like 0 active bands) allows bypassing the outer processing loop entirely.
