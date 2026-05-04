## 2024-05-04 - Initial Setup
**Learning:** Found an anti-pattern in the frontend using setInterval to poll the backend (`get_visualizer_data`). If the backend takes longer than the interval (50ms), executions will overlap causing performance issues and potential state problems.
**Action:** Replace `setInterval` with a recursive `setTimeout` pattern when polling async Tauri endpoints.
