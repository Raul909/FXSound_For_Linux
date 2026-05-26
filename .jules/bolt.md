
## 2026-05-26 - Prevent Redundant Loops on Flat State
**Learning:** Even when inner loop iterations are 0, iterating over a large audio buffer to perform identity operations (e.g., modulo math and redundant memory writes) introduces unnecessary CPU overhead and cache churn in high-frequency DSP paths.
**Action:** Always add early returns after state evaluation (e.g., `active_count == 0`) to entirely bypass O(N) buffer iteration when the effect state is inactive or flat.
