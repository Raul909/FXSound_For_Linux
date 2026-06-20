## 2024-05-18 - Optimize Silence Detection

**Learning:** In the hot audio loop, replacing the Root Mean Square (RMS) calculation with Mean Absolute Value (MAV) approximation for silence detection yields a noticeable speedup. It avoids per-sample floating-point multiplication and the relatively expensive `sqrt()` operation.
**Action:** Replace `RMS` calculations with `MAV` where an exact RMS value is not strictly required.
