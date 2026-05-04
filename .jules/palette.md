
## 2026-05-04 - Keyboard Accessibility for Custom Sliders
**Learning:** Custom slider components (`EQBand`, `EffectSlider`) built with `div` elements lack native keyboard navigation (`tabIndex`, `onKeyDown`) and screen reader support (`role="slider"`, `aria-valuenow`). This pattern makes core app functionality inaccessible to non-mouse users.
**Action:** Always add `tabIndex={0}`, `role="slider"`, appropriate ARIA attributes, and `onKeyDown` handlers with arrow key support when implementing custom draggable UI elements. Ensure `:focus-visible` styles are added to match hover states.
