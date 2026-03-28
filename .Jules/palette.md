## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.

## 2024-03-28 - Make custom sliders keyboard accessible
**Learning:** In React components, custom UI elements built with <div> (e.g., sliders) must implement full a11y: role="slider", dynamic tabIndex (e.g., disabled ? -1 : 0), aria-valuenow/min/max, aria-orientation (if vertical), and W3C-compliant keyboard handlers (Arrows, Home for min, End for max).
**Action:** When creating or modifying custom sliders, ensure all ARIA properties are present and keyboard support is provided for navigation and value adjustment.
