## 2026-06-06 - Replaced RMS with MAV for silence detection
**Learning:** Calculating RMS to detect silence is an expensive O(N) operation due to per-sample multiplications and a square root operation.
**Action:** Used Mean Absolute Value (MAV) instead for an approximation, significantly reducing processing overhead per sample buffer.
