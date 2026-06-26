# Bolt's Journal
## 2024-05-24 - Tauri IPC Batching
**Learning:** Sending multiple state updates sequentially via `invoke` across the Tauri bridge can cause noticeable overhead, especially when updating full arrays of state (like an EQ preset with 10 bands and 5 effects, which triggers 15 IPC calls).
**Action:** When updating a large object or related set of fields, create a batched backend command to receive the full state at once instead of making many granular IPC calls.
