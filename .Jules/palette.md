## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.

## 2026-04-19 - Accessible Slider Implementation
**Learning:** Custom UI components built with <div>s (like sliders) require manual implementation of full accessibility support, including role="slider", dynamic tabIndex for focusability, aria-value attributes for screen readers, and explicit keyboard event handlers (Arrows, Home, End) with event.preventDefault() to prevent unintended scrolling.
**Action:** Ensure all custom interactive widgets receive standard keyboard and screen reader support, replicating native input behaviors to maintain inclusive UX.
