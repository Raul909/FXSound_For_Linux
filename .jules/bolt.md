## 2026-06-06 - Replaced RMS with MAV for Audio Silence Detection
**Learning:** In hot audio DSP loops, calculating Root Mean Square (RMS) incurs unnecessary CPU overhead due to per-sample multiplications and expensive global square root math, especially when only used for simple thresholding.
**Action:** Used Mean Absolute Value (MAV) as a fast, multiplication-free approximation for silence detection (`mav < 0.0009` instead of `rms.sqrt() < 0.001`), significantly reducing processing time.
