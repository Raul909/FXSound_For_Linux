## 2026-05-11 - Early Returns in DSP Loops
**Learning:** DSP loops that process arrays of samples can still incur overhead from loop machinery, array indexing, modulo arithmetic, and redundant memory writes even when the inner processing logic is skipped (e.g., when active filter count is 0).
**Action:** Always add early returns before O(N) DSP loops when the effect state is flat/inactive to completely skip unnecessary buffer iterations.
