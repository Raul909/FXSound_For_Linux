
## 2026-05-16 - Custom Sliders Keyboard Accessibility
**Learning:** Custom div-based sliders (like `EffectSlider` and `EQBand`) are entirely invisible to keyboard and screen reader users by default. Implementing ARIA slider roles requires handling directional keys (ArrowUp/Down/Left/Right) and edge-cases (Home/End), as well as pairing `:focus-visible` with existing `:hover` states for visible focus indicators without adding new CSS classes.
**Action:** Always verify `tabIndex`, `role="slider"`, `aria-valuenow`/`min`/`max`, and `onKeyDown` are present when building custom interactive components, and mirror `:hover` styles to `:focus-visible` for consistent focus states.
