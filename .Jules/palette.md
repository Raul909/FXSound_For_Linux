
## 2026-05-10 - Keyboard accessibility for custom sliders
**Learning:** Custom interactive elements (like vertical and horizontal sliders) using mouse drag events are opaque to screen readers and keyboard users by default.
**Action:** Added `role="slider"`, `aria-valuenow`, `aria-valuemin`, `aria-valuemax`, and `tabIndex={0}` to the custom slider tracks to make them accessible and discoverable. Bound `ArrowUp`/`ArrowDown` and `ArrowLeft`/`ArrowRight` keydown events to adjust values, mirroring native `<input type="range">` behavior. Also added `:focus-visible` CSS styles mirroring `:hover` states for visible keyboard focus indicators without adding custom classes.
