## 2026-05-15 - Accessibility for Custom Div-Based Sliders
**Learning:** Custom div-based slider tracks are completely invisible to keyboard and screen-reader users by default.
**Action:** Always implement role="slider", an appropriate tabIndex, ARIA attributes (aria-valuenow, aria-valuemin, aria-valuemax, aria-label), and an onKeyDown handler for directional arrow keys when building non-native slider components. Ensure focus indicators are preserved by combining :focus-visible with existing :hover CSS states.
