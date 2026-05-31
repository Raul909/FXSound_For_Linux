## 2026-05-31 - Add early return for flat EQ bands in Rust DSP
**Learning:** Found a missing early return in the Rust DSP pipeline (`apply_eq` in `audio.rs`). Even when no EQ bands were active (flat EQ), the code still iterated over all audio samples in the buffer, re-assigning values in a no-op loop on every buffer tick.
**Action:** Always check for early return opportunities before entering expensive O(N) sample processing loops, especially when the effect or filter states can be completely inactive.
