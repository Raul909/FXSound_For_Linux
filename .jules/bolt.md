## 2026-06-05 - Add early return to skip empty EQ loop
**Learning:** In tight DSP audio loops (like apply_eq), skipping O(N) sample processing completely when an effect is inactive (e.g. flat EQ) saves unnecessary memory read/write iterations and CPU cycles.
**Action:** In DSP loops, use early returns (`if active_count == 0`) to skip O(N) audio sample processing loops and prevent redundant memory writes entirely when the effect state is flat/inactive.
