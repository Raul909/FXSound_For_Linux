## 2026-05-06 - [Early Return in DSP Loops]
**Learning:** The `apply_eq` DSP loop iterates over every single audio sample (O(N) complexity). When no EQ bands are active (flat state), this processing is completely redundant and wastes CPU cycles.
**Action:** Always verify if a DSP loop or expensive operation can be bypassed entirely with an early return (e.g., `if active_count == 0`) before executing the per-sample iteration.
