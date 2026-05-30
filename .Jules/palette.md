## 2026-05-30 - Keyboard Accessibility for Custom Sliders
**Learning:** Custom div-based sliders (`.eq-band__track` and `.effect-slider__track`) lacked native keyboard and screen reader accessibility, rendering them unusable without a mouse.
**Action:** Implemented `tabIndex`, ARIA roles (`role="slider"`, `aria-valuenow`, `aria-valuemin`, `aria-valuemax`), and `onKeyDown` handlers for directional arrow key interaction. Appended `:focus-visible` to existing hover CSS with `outline: none;` to display visible focus indicators without visual clashing.
