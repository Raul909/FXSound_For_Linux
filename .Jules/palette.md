## 2026-05-19 - Manual Accessibility for Custom Sliders
**Learning:** Custom div-based sliders require manual implementation of accessibility features (role="slider", tabIndex, aria attributes, and keyboard event handlers) to function correctly for keyboard-only and screen reader users. Focus visibility can be added efficiently by appending `:focus-visible` to existing `:hover` CSS rules.
**Action:** Always verify if custom interactive components have adequate keyboard support and ARIA attributes during implementation.
