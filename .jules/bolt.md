## 2026-06-07 - Replaced RMS with MAV for audio silence detection
**Learning:** Calculating Root Mean Square (RMS) for silence detection in high-frequency audio DSP loops introduces unnecessary overhead due to per-sample multiplications and global square root calculations.
**Action:** Use Mean Absolute Value (MAV) with a calibrated threshold (e.g., `mav < 0.0009` instead of `rms.sqrt() < 0.001`) as a faster, computationally cheaper approximation for silence detection.
