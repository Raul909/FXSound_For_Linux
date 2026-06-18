## 2026-06-18 - Optimize silence detection
**Learning:** In audio processing loops, calculating RMS (Root Mean Square) for silence detection is expensive due to the square and square-root operations applied per-sample, which are processed thousands of times a second.
**Action:** Replace RMS calculation with Mean Absolute Value (MAV) for silence detection. MAV provides a close enough approximation of signal energy for silence thresholding but only requires `.abs()`, eliminating the expensive multiplications and the final `sqrt()` calculation.
