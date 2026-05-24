
## 2026-05-24 - Custom Sliders Keyboard Accessibility
**Learning:** Custom div-based sliders require manual implementation of `role="slider"`, `tabIndex`, `aria-valuenow/min/max`, and `onKeyDown` (with arrow keys) to be usable by keyboard-only and screen reader users. Additionally, reusing `:hover` styles with `:focus-visible` provides a clean focus state without custom CSS classes, provided `outline: none;` is set.
**Action:** Always implement full ARIA attributes and keyboard handlers when building custom interactive components like sliders, and map hover effects to focus-visible.
