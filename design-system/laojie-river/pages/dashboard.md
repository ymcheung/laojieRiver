# Dashboard Page Overrides

> **PROJECT:** LaoJie River
> **Generated:** 2026-06-04 22:39:46
> **Page Type:** Dashboard / Data View

> ⚠️ **IMPORTANT:** Rules in this file **override** the Master file (`design-system/laojie-river/MASTER.md`).
> Only deviations from the Master are documented here. For all other rules, refer to the Master.

---

## Page-Specific Rules

### Layout Overrides

- **Max Width:** No marketing max-width cap for the unlocked app shell. Use the full desktop window with comfortable internal constraints.
- **Layout:** Desktop app layout: persistent left sidebar, top command/search area, vault item list, and detail pane. Collapse to list/detail navigation at narrow widths.
- **Wireframe Frame:** Use a centered split-pane workspace for vault and settings screens: left pane, explicit Zinc divider, broad right pane, rounded top-left corner on the left pane, and flat Zinc page background.
- **Right Edge:** On desktop, the split-pane workspace starts at the wireframe-aligned left position and extends to the app viewport's right edge. Do not center it with right-side whitespace.
- **Resizable Divider:** On desktop, the vertical Zinc bar between the panes is an interactive splitter. Support pointer drag and keyboard arrow resizing, and clamp pane widths so the left pane remains at least 320px and the right pane remains at least 560px.
- **Vault Navigation Placement:** The All Items/Favorites/Logins/Secure Notes/Cards/Identities/2FA navigation sits outside the split-pane workspace as a separate left rail on desktop. Do not place this navigation inside the left content pane.
- **Sections:** Navigation groups, search/command bar, add/lock controls, filtered item list, selected item detail, and transient status/toast region.

### Spacing Overrides

- No overrides — use Master spacing

### Typography Overrides

- Use readable sans typography for all dashboard UI. TOTP codes, timers, timestamps, and security metadata should use tabular numerals.

### Color Overrides

- **Strategy:** Tailwind Zinc surfaces with Tailwind Sky primary action, selection, focus, and controlled emphasis. Do not use Slate, Gray, Amber, Green, Red, Purple, or custom chromatic status ramps.
- **Status Handling:** Expiry, warning, success, destructive, failed-security, locked, and copied states use Zinc text/surfaces/borders plus explicit labels, icon shape, confirmation steps, placement, and motion. Sky can show focus or the active action, but it must not replace clear security wording.
- **TOTP Timing:** Near-expiry and expired states should shift copy, icon treatment, ring density, or progress pattern within Zinc tokens. Avoid amber/green/red countdown colors.

### Component Overrides

- Require: Password reveal controls with explicit labels and focus-visible states
- Avoid: Low-contrast Zinc text on Zinc backgrounds
- Avoid: Low contrast text
- Avoid: Hero, feature-card, CTA, testimonial, or landing-page composition inside the vault dashboard

---

## Page-Specific Components

- Sidebar navigation
- Command/search menu
- Vault item list
- Item detail panel
- Vault empty state
- Password reveal/copy controls
- TOTP timer ring
- Strength meter
- Auto-lock/status toast
- Alert dialogs for delete/export/security-sensitive changes

---

## Recommendations

- Effects: Short opacity/color transitions only; no layout-shifting hover transforms or decorative grain/noise.
- Forms: Toggle to show/hide password, label every control, and keep account-password and vault-master-password forms visually distinct.
- Typography: Use Zinc text with strong contrast on Zinc surfaces; use tabular numerals for TOTP and timestamps.
- Accessibility: Minimum 4.5:1 ratio for normal text, visible keyboard focus, Escape-friendly overlays, and screen-reader labels for icon buttons.
- Command UX: Cmd/Ctrl+K should find items and actions such as add login, generate password, open settings, and lock vault.
- Empty States: Use concise, task-specific states for empty filters, first-run vaults, and unselected detail panes. Keep actions visible, avoid blank panels, and use Sky only for the primary action/focus while the container, icon frame, borders, and text remain Zinc.
- Settings Entry: Keep the top-right `Settings` text at the same size as the vault action buttons (`text-sm` / 14px). Do not promote it to a page title scale.
