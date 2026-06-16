## 2024-06-16 - Optimize canvas rendering in React components
**Learning:** Avoid creating objects like CanvasGradient or recalculating colors inside high-frequency requestAnimationFrame draw loops in React components (e.g., Visualizer.jsx). This causes per-frame object allocation and garbage collection overhead.
**Action:** Cache these values (e.g., via useMemo) when their dependencies change to minimize garbage collection overhead.
