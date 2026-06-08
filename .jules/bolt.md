## 2026-06-08 - MAV approximation for silence detection
**Learning:** In audio DSP optimization, using RMS (Root Mean Square) for silence detection requires computationally expensive global square roots and per-sample multiplications (input.iter().map(|&x| x * x).sum::<f32>() / len, then .sqrt()). Using Mean Absolute Value (MAV) is a faster approximation for silence detection (input.iter().map(|x| x.abs()).sum::<f32>() / len).
**Action:** Replace rms.sqrt() < 0.001 with mav < 0.0009 in process_audio for faster silence detection.
