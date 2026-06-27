## 2024-06-27 - Visualizer Render Optimization
**Learning:** In a canvas-based animation loop that runs continually (like an audio visualizer), simply adding an early return to bypass rendering can permanently kill the animation if the `requestAnimationFrame` call is located at the bottom of the function.
**Action:** When adding an early return to an animation loop, ensure that the background polling or next frame request (`setTimeout` or `requestAnimationFrame`) is called *before* the return, so the loop can resume when the condition changes.
