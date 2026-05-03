## 2024-05-03 - DSP Loop Early Return
**Learning:** In audio processing, iterating over a buffer of samples even to do nothing (`*sample = *sample`) consumes measurable CPU cache bandwidth.
**Action:** Always add early returns in DSP loops to skip processing entirely when the effect/EQ is bypassed or flat.
