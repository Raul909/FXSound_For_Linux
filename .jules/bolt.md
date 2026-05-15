
## 2026-05-15 - Add early return to skip redundant DSP processing in apply_eq
**Learning:** In audio DSP loops, even if inner filter loops do not execute (e.g., active_count == 0), the outer loop over all audio samples can still cause redundant memory reads/writes and waste CPU cycles.
**Action:** Always employ early returns before O(N) sample processing loops when the state is flat or inactive to entirely prevent redundant buffer iterations and memory writes.
