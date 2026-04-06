## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.

## 2024-04-06 - Accessible Custom Sliders
**Learning:** Custom slider components built with `<div>` require `role="slider"`, `tabIndex`, ARIA values (`min`, `max`, `now`), keyboard event handlers, and `:focus-visible` styling to be accessible.
**Action:** Always ensure that custom inputs implement appropriate ARIA roles and full keyboard support (including Arrow keys, Home, and End).
