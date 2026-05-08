## 2026-05-08 - Add visible focus indicator with :focus-visible
**Learning:** The :focus-visible pseudo-class can be utilized when custom CSS classes are restricted but focus ring visibility is required for keyboard navigation, preserving mouse users' experience.
**Action:** When adding tabIndex to custom UI controls, always ensure visible focus styles are provided for :focus-visible in conjunction with ARIA roles to support screen readers and keyboard users.
