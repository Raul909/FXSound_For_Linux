## 2026-06-01 - Early Return in DSP Loops
**Learning:** Even when the inner loops (like applying inactive biquad filters) do no work, iterating over an entire audio buffer array in a hot path causes redundant memory operations and wastes CPU cycles.
**Action:** Add early returns (e.g., `if active_count == 0`) before O(N) audio sample processing loops to skip the processing entirely when all effect states are inactive.
