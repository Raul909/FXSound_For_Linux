## 2026-05-03 - Custom Sliders Keyboard Accessibility
**Learning:** Custom interactive elements built with `<div>` tags completely bypass browser keyboard accessibility out of the box. Users relying on keyboard navigation cannot focus on them or adjust their values.
**Action:** Always add `tabIndex`, ARIA roles (`role="slider"`), ARIA properties, and custom `onKeyDown` handlers to custom div-based interactive components to ensure they can be focused and operated via the keyboard. Implement `:focus-visible` styles to provide a visual focus indicator.
