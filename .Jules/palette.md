## 2026-05-22 - Custom Div Sliders Keyboard Accessibility
**Learning:** Custom div-based sliders require manual implementation of ARIA state and keyboard handlers to be accessible. Combining `outline: none` with `:focus-visible` is an effective way to match custom focus states to existing `:hover` styles.
**Action:** Always add `role="slider"`, `tabIndex`, `aria-valuenow/min/max`, and `onKeyDown` handlers for Arrow up/down/left/right navigation when building or updating custom sliders.
