## 2026-05-29 - Accessible Custom Sliders
**Learning:** Custom div-based sliders need manual ARIA roles (`slider`), keyboard handlers (`onKeyDown`), and focus indicators (`tabIndex={0}`) to be usable by screen readers and keyboard navigation. Existing CSS `:hover` styles can be reused with `:focus-visible` to provide focus states without adding new classes.
**Action:** Always implement `role="slider"`, `tabIndex`, `aria-valuenow`/`min`/`max`, and arrow key support when creating non-native slider components. Use `:focus-visible` to map hover styles to focus states.
