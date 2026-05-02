## 2026-05-02 - Keyboard accessibility on custom slider tracks
**Learning:** Custom slider elements (divs) lack built-in keyboard accessibility and ARIA roles. While they can be made interactive via mouse events, they need a `role="slider"`, `tabIndex`, ARIA values, and an `onKeyDown` handler to allow screen readers and keyboard users to properly interact with them.
**Action:** Always add keyboard interaction (Arrow keys) and `tabIndex` to custom UI components that mimic native input controls.
