## 2026-06-04 - Precalculate Canvas Properties
**Learning:** In high-frequency `requestAnimationFrame` draw loops like in `Visualizer.jsx`, recalculating static color properties (hue, saturation) for every bar on every frame creates unnecessary CPU load.
**Action:** Precalculate and cache static values outside the frame loop or in a `useRef` to reduce per-frame computations.
