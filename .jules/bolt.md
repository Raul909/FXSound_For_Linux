## 2024-05-17 - Recursive setTimeout vs setInterval for backend polling
**Learning:** Using `setInterval` for polling asynchronous Tauri endpoints (e.g., `invoke('get_visualizer_data')`) can cause overlapping executions and performance issues if the backend takes longer than the interval to respond. This is especially true for visualizers running at high frequencies.
**Action:** Replace `setInterval` with a recursive `setTimeout` pattern to ensure the next poll only starts after the previous one has completely finished, preventing request pileups.

## 2024-05-17 - Biquad loop inversion for CPU cache locality
**Learning:** In DSP loops like `apply_eq` within `src-tauri/src/audio.rs`, processing the entire audio buffer through each filter sequentially causes repeated memory access to the same buffer. This leads to poor CPU cache locality and increased memory bandwidth overhead.
**Action:** Employ loop inversion/fusion: process each audio sample completely through all active filters before moving to the next sample, rather than iterating over the entire buffer for each filter. This is already implemented in `apply_eq`.

## 2026-05-02 - Flat EQ bands early return
**Learning:** The `apply_eq` DSP loop evaluates whether filters are active or not. If no EQ bands are active (flat state), we can skip the expensive O(N) audio sample processing loop entirely.
**Action:** Use an early return (`if active_count == 0`) to skip the loop and save CPU cycles when the EQ is flat.
