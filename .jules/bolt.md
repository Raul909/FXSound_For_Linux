# Bolt's Journal
## 2024-05-24 - Tauri IPC Batching
**Learning:** Sending multiple state updates sequentially via `invoke` across the Tauri bridge can cause noticeable overhead, especially when updating full arrays of state (like an EQ preset with 10 bands and 5 effects, which triggers 15 IPC calls).
**Action:** When updating a large object or related set of fields, create a batched backend command to receive the full state at once instead of making many granular IPC calls.

## 2024-06-27 - Canvas Animation Redraw Loop
**Learning:** `requestAnimationFrame` drawing loops in canvas components can consume significant CPU/GPU resources even when the data is unchanged (e.g., during silent or idle periods). Asymptotic interpolation without a threshold means values never truly reach their targets, preventing the loop from resting.
**Action:** Always include a `needsDraw` flag to skip the clear-and-draw phase when data hasn't visually changed. Snap asymptotically approaching values to their targets when the difference falls below a tiny threshold (e.g., < 0.05).
## 2024-06-27 - Visualizer Render Optimization
**Learning:** In a canvas-based animation loop that runs continually (like an audio visualizer), simply adding an early return to bypass rendering can permanently kill the animation if the `requestAnimationFrame` call is located at the bottom of the function.
**Action:** When adding an early return to an animation loop, ensure that the background polling or next frame request (`setTimeout` or `requestAnimationFrame`) is called *before* the return, so the loop can resume when the condition changes.
## 2026-06-28 - Optimize Visualizer Rendering Loop
**Learning:** In canvas-based animation loops using `requestAnimationFrame`, simple asymptotic interpolation (e.g., `val += (target - val) * speed`) will never mathematically reach the target, leading to continuous micro-updates and preventing the loop from resting. This wastes CPU/GPU cycles.
**Action:** When interpolating values towards a target, always calculate the difference first. If the difference is below a small visual threshold (like `0.05`), snap the value exactly to the target to allow the rendering loop to pause.

## 2026-06-29 - Throttle mousemove with requestAnimationFrame
**Learning:** High-polling mice (up to 1000Hz) trigger `mousemove` events far faster than the browser can render them (usually 60-144Hz). When these events directly trigger React state updates or expensive operations (like Tauri IPC calls), it causes main-thread bottlenecking and excessive backend load.
**Action:** Always throttle continuous UI events (like `mousemove` or `scroll`) using `requestAnimationFrame`. Cache the latest event coordinates and only process the update once per frame. Ensure to flush the final state on `mouseup`/`touchend` to not lose the last update.

## 2024-07-01 - Prevent DOM Thrashing on Tab Switches
**Learning:** In complex React applications, completely unmounting heavy components (like the 15 sliders across two tabs in this codebase) during simple tab switches causes significant layout thrashing and Garbage Collection spikes. Conditional rendering (`{tab === 'eq' && <EqTab/>}`) is a React anti-pattern for frequently toggled heavy UI elements.
**Action:** Use CSS `display: none` (`style={{ display: isActive ? "block" : "none" }}`) instead of conditional rendering for tabs that contain many DOM elements or complex interactive components, so the DOM tree is preserved across tab transitions.
