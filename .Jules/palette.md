## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.
## 2026-04-01 - Add custom keyboard handlers for sliders
**Learning:** When making custom -based sliders accessible, implementing standard keyboard handlers (Arrow keys, Home, End) is crucial for usability, and supporting  for larger step increments is a recommended W3C pattern that significantly improves UX.
**Action:** Add full ARIA attributes (, , ) along with customized  handlers supporting Shift-modified steps for any future custom UI controls.
## 2026-04-01 - Custom slider accessibility
**Learning:** When building custom div sliders, supporting keyboard navigation (Arrow keys, Home, End) and adding aria-valuenow, aria-valuemin, aria-valuemax, and role="slider" is essential for screen readers and keyboard users.
**Action:** Always add tabIndex, ARIA slider properties, and keyboard handlers to custom interactive input UI.
