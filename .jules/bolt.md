## 2026-06-18 - Optimize silence detection with Mean Absolute Value
**Learning:** In audio processing, calculating RMS (Root Mean Square) for silence detection is expensive due to the per-sample multiplication `x * x` and the global `sqrt()` call.
**Action:** Replaced RMS with Mean Absolute Value (MAV) for silence detection. MAV `x.abs()` is significantly cheaper and serves the exact same thresholding purpose when scaled appropriately (`0.0009` MAV ≈ `0.001` RMS).
