## 2024-05-24 - Optimize Silence Detection using MAV
**Learning:** In hot audio processing loops (e.g., `process_audio`), calculating RMS for silence detection incurs significant overhead due to per-sample multiplications and a global square root calculation.
**Action:** Replaced RMS with Mean Absolute Value (MAV) approximation (`mav < 0.0009`). This eliminates expensive math operations while maintaining reliable silence gating.
## 2026-06-12 - Optimize DSP loops with early returns
**Learning:** In DSP loops (e.g., apply_eq), iterating over arrays when effects are flat/inactive causes redundant memory reads and writes, resulting in unnecessary overhead.
**Action:** Use early returns (if active_count == 0) to skip expensive O(N) audio sample processing loops entirely when the effect state is inactive.
