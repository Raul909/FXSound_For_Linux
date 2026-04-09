## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.

## 2024-03-06 - Add complete a11y to custom React sliders
**Learning:** In the React frontend, custom UI components built with `<div>` (e.g., sliders) must implement full a11y: `role="slider"`, dynamic `tabIndex`, `aria-valuenow`/`min`/`max`, `aria-orientation`, W3C-compliant keyboard handlers (Arrows, Home, End), and explicit `:focus-visible` styling in CSS for keyboard navigation.
**Action:** When creating custom inputs, always include a full keyboard handler, appropriate ARIA properties to expose state, and clear visual focus indicators.
