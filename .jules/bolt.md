## 2026-06-15 - Audio loop silence detection optimization
**Learning:** Calculating Root Mean Square (RMS) for silence detection on the hot audio path introduces unnecessary overhead due to per-sample multiplications and a global square root calculation. Mean Absolute Value (MAV) provides a sufficiently accurate and much faster approximation.
**Action:** Replace RMS with MAV in the audio processing loop to eliminate expensive math operations.
