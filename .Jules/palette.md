## 2026-05-05 - Keyboard Accessibility for Custom Sliders
**Learning:** Custom components built with `<div>` and native drag handlers completely exclude keyboard users unless explicitly managed. Appending `:focus-visible` to existing `:hover` rules allows adding a clear focus indicator without violating design constraints by introducing new CSS classes.
**Action:** Always add `tabIndex`, `role="slider"`, ARIA attributes (`aria-valuenow`, etc.), and an `onKeyDown` handler for arrow key support when building custom sliders.
