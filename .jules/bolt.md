## 2026-05-01 - Prevent overlapping execution on async Tauri polling
**Learning:** Using `setInterval` for polling asynchronous endpoints (e.g., `invoke('get_visualizer_data')`) in the React frontend can cause overlapping executions if the backend takes longer than the interval to respond. This leads to redundant concurrent requests, CPU thrashing, and degraded performance.
**Action:** Use a recursive `setTimeout` pattern (e.g., calling `setTimeout(pollBackend, 50)` inside `pollBackend`) to ensure the next poll is only scheduled after the previous asynchronous request fully completes.
