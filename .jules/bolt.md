# Bolt's Journal
## 2024-05-24 - Tauri IPC Batching
**Learning:** Sending multiple state updates sequentially via `invoke` across the Tauri bridge can cause noticeable overhead, especially when updating full arrays of state (like an EQ preset with 10 bands and 5 effects, which triggers 15 IPC calls).
**Action:** When updating a large object or related set of fields, create a batched backend command to receive the full state at once instead of making many granular IPC calls.

## 2024-06-27 - Canvas Animation Redraw Loop
**Learning:** `requestAnimationFrame` drawing loops in canvas components can consume significant CPU/GPU resources even when the data is unchanged (e.g., during silent or idle periods). Asymptotic interpolation without a threshold means values never truly reach their targets, preventing the loop from resting.
**Action:** Always include a `needsDraw` flag to skip the clear-and-draw phase when data hasn't visually changed. Snap asymptotically approaching values to their targets when the difference falls below a tiny threshold (e.g., < 0.05).
