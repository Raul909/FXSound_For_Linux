## 2026-05-22 - Add early return in apply_eq
**Learning:** In DSP loops like apply_eq, even an empty active slice can cause an expensive O(N) loop iterating over every sample (including modulo arithmetic and redundant memory writes) if no early return is present.
**Action:** Always add an early return (`if active_count == 0`) before sample-processing loops to entirely skip the processing overhead when the effect state is flat/inactive.
