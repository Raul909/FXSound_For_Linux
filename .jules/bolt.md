## 2026-05-10 - Optimize DSP Effects Pipeline via Loop Fusion
**Learning:** In audio DSP processing, sequential array passes for multiple independent effects cause redundant memory reads/writes and reduce CPU cache efficiency.
**Action:** Use loop fusion to combine multiple effect passes into a single iteration over the audio buffer, pre-calculating constants outside the loop to minimize floating-point math per sample.
