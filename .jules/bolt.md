## 2026-06-16 - Fast Silence Detection via Mean Absolute Value
**Learning:** In the audio processing pipeline (`src-tauri/src/audio.rs`), calculating the Root Mean Square (RMS) to detect silence is unnecessarily expensive due to per-sample multiplications and a global square root calculation.
**Action:** Replace RMS with a Mean Absolute Value (MAV) approximation (`mav < 0.0009`), which is significantly faster and approximately equivalent to `rms < 0.001` for audio signals.
