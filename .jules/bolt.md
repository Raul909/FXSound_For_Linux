## 2026-06-12 - Cache Visualizer Gradients
**Learning:** Creating CanvasGradient and recalculating colors inside high-frequency requestAnimationFrame draw loops causes high garbage collection overhead and drops frames. Pre-calculating and caching these values based on discrete heights eliminates this overhead.
**Action:** Always cache complex objects like CanvasGradient and frequently calculated styles if their possible states are bounded (e.g. integer heights) to optimize requestAnimationFrame rendering.
