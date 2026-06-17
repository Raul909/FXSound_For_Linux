## 2024-06-17 - Visualizer Component Memoization
**Learning:** In highly dynamic React components (like the audio visualizer running at 60fps), recalculating gradients and objects within a `useCallback` loop causes significant garbage collection overhead, even if they aren't tied to React's state tree.
**Action:** Move expensive object allocations (like `CanvasGradient`) into a `useMemo` block that explicitly pre-computes them for all possible states (e.g. 0 to 100 height) when dependencies change, and just reference the array index on each frame.
