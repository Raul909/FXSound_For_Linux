## 2026-06-15 - Replace RMS with MAV for silence detection
**Learning:** In real-time audio processing loops, calculating the Root Mean Square (RMS) requires expensive per-sample multiplications and a global square root calculation.
**Action:** Use Mean Absolute Value (MAV) approximation instead of RMS for simple thresholding tasks like silence detection to eliminate unnecessary floating-point operations.
