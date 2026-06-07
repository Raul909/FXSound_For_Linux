## 2026-06-07 - Keyboard Accessible Custom Sliders
**Learning:** Custom div-based sliders in React require manual implementation of `role="slider"`, `tabIndex`, ARIA state attributes, and keyboard event handlers with `event.preventDefault()` to function correctly for keyboard and screen reader users.
**Action:** When creating custom interactive elements, always ensure keyboard accessibility by adding appropriate ARIA roles, states, `tabIndex`, keyboard event listeners, and visible focus indicators using `:focus-visible` paired with `outline: none;`.
