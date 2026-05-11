## 2026-05-11 - Custom Slider Keyboard Accessibility
**Learning:** Custom interactive components like div-based sliders lack built-in keyboard navigation and ARIA roles by default, rendering them inaccessible to screen readers and keyboard-only users.
**Action:** Always provide role="slider", tabIndex, aria-valuenow/min/max, and implement onKeyDown handlers with focus-visible styling for custom interactive controls.
