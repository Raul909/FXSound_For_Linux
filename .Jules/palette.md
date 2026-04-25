## 2024-03-06 - Improve accessibility for icon-only buttons and labeled selects
**Learning:** In React implementations where generic `<div>`s are used as visual labels next to interactive elements, screen readers will completely fail to associate them unless explicitly linked using `id` and `aria-labelledby`. Also, icon-only buttons that convey binary state (like power) must combine `aria-label` with `aria-pressed` to correctly indicate both function and current status to assistive technologies.
**Action:** When adding or reviewing custom interactive components (like dropdowns and status toggles), verify that visual text is explicitly linked to `<select>` or `<input>` tags using proper ARIA attributes, and ensure all purely visual icon buttons have corresponding textual alternatives and explicit state announcements.

## 2026-04-25 - Custom Slider Keyboard Accessibility
**Learning:** Custom UI components built with `<div>` (like EffectSlider) completely lack native focus and keyboard interaction, rendering them invisible to screen readers and keyboard users.
**Action:** Always implement full W3C slider a11y (`role="slider"`, `tabIndex`, dynamic `aria-valuenow`, Arrow/Home/End keyboard handlers) and explicit `:focus-visible` styles for non-native interactive elements.
