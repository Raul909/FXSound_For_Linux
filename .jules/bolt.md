## 2026-05-29 - Optimize DSP loops with fusion and early returns
**Learning:** Chaining DSP effects often results in redundant loop iterations over the same buffer, causing memory bandwidth waste and cache thrashing. Loop fusion combined with pre-calculating constants outside the loop significantly reduces O(N) overhead.
**Action:** In high-frequency processing paths, check for early returns when effects are inactive, pre-calculate constant math outside the main loop, and fuse sequential buffer traversals into a single pass per sample.
