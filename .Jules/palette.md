
## 2026-05-06 - Keyboard Accessibility for Custom Sliders
**Learning:** Custom interactive components like sliders often miss accessibility by default because `div` tags aren't inherently focusable. Reusing existing `:hover` states by appending `:focus-visible` to them allows adding visible focus indicators without breaking UX constraints that forbid entirely new custom CSS classes.
**Action:** Always add `tabIndex`, ARIA roles/values, and `:focus-visible` styling (mirroring hover states) to custom interactive components that rely on `onMouseDown`.
