## 2026-06-07 - Replace RMS with MAV for Silence Detection
**Learning:** In high-frequency DSP loops (like audio processing), calculating exact Root Mean Square (RMS) involves expensive per-sample multiplications (`x * x`) and a final square root calculation.
**Action:** Use Mean Absolute Value (MAV) approximation (`x.abs()`) instead of RMS for simple threshold checks (like silence detection) to eliminate these expensive operations and reduce CPU usage.
