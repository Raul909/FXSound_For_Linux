## 2026-06-13 - O(N) Loop Optimization in Audio Processing
**Learning:** Even if a loop's inner operations are skipped, iterating over thousands of audio samples and executing redundant memory writes still introduces measurable CPU overhead in high-frequency DSP paths.
**Action:** Implement early returns (e.g., `if active_count == 0`) before O(N) sample loops to completely bypass iteration overhead when the DSP state is inactive.
