## YYYY-MM-DD - [Title]
**Learning:** [Insight]
**Action:** [How to apply next time]
## 2026-04-23 - Prevent overlapping async executions
**Learning:** Using `setInterval` for asynchronous polling (like Tauri IPC calls) can cause overlapping executions and performance degradation if the operation takes longer than the interval.
**Action:** Always use a recursive `setTimeout` pattern for repeated asynchronous operations to ensure the next execution only begins after the previous one finishes.
