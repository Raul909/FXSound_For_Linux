## 2026-06-19 - Custom Slider Accessibility
**Learning:** Custom div-based sliders require manual implementation of ARIA roles, tabIndex, and keyboard handlers to be accessible for keyboard-only and screen reader users. Additionally, :focus-visible must be explicitly styled to prevent the browser's default outline from clashing with the custom focus ring.
**Action:** Always implement role="slider", tabIndex, aria-value attributes, and keyboard handlers when building custom UI controls. Map :focus-visible to existing :hover states for consistent visual feedback.
