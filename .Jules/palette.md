## 2026-06-05 - Add keyboard accessibility to custom sliders
**Learning:** Custom div-based sliders completely block keyboard-only and screen reader users from interacting with core features (EQ and Effects) since they lack native slider behaviors.
**Action:** Always add `role="slider"`, `tabIndex`, ARIA min/max/valuenow attributes, and `onKeyDown` handlers for arrow keys when building custom interactive elements, and apply `:focus-visible` to match existing hover states for visible focus.
