
## 2026-05-20 - Accessible Custom Sliders
**Learning:** Custom div-based sliders like `EQBand` and `EffectSlider` inherently lack keyboard navigation and semantic meaning, blocking screen readers and keyboard users from interacting with core DSP features.
**Action:** When building custom interactive components, manually implement `role="slider"`, appropriate ARIA state attributes (`aria-valuenow`, min, max), `tabIndex`, and a keyboard event handler (`onKeyDown`) to ensure they behave like native `<input type="range">`. Added `:focus-visible` to `:hover` styles to maintain visual indicators without introducing custom classes.
