## 2026-05-25 - Keyboard Accessible Custom Sliders
**Learning:** Custom div-based sliders lack native keyboard operability and screen reader context. Using `tabIndex`, explicit `onKeyDown` listeners for arrows, and ARIA attributes (`role="slider"`, `aria-valuenow`) is required to make them accessible.
**Action:** When creating custom interactive components (like sliders or toggles), always manually implement keyboard navigation handlers and ARIA attributes, and ensure visual focus indicators are present.
