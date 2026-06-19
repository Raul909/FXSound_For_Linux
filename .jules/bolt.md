## 2026-06-19 - Optimize silence detection with Mean Absolute Value
**Learning:** In hot audio processing loops, calculating RMS (Root Mean Square) is expensive due to per-sample multiplications and a global square root. Mean Absolute Value (MAV) provides a sufficient approximation for simple silence detection and is significantly faster.
**Action:** Replace RMS with MAV (`x.abs()` sum) in audio processing paths where exact power measurement isn't necessary.
