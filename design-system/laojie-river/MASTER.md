# Design System Master File

> **LOGIC:** When building a specific page, first check `design-system/laojie-river/pages/[page-name].md`.
> If that file exists, its rules **override** this Master file.
> If not, strictly follow the rules below.

---

**Project:** LaoJie River
**Generated:** 2026-06-04 22:39:46
**Category:** Secure Local-First Desktop Password Manager / TOTP Vault

---

## Repo Application Notes

This generated master file is a starting point from the `ui-ux-pro-max` search. For this repo, apply it through the PRD lens: LaoJie River is a secure local-first Tauri desktop vault, not a marketing site or generic CRUD app.

- Treat the dashboard as an authenticated work surface. Do not use hero sections, sticky marketing CTAs, testimonial patterns, or decorative landing-page composition inside the app shell.
- Support Zinc-based light and dark themes. Prefer following the OS theme on first launch; dark mode should be polished, but light mode is not forbidden.
- Use Tailwind Sky as the only chromatic theme family. All non-theme UI color must come from Tailwind Zinc, including surfaces, text, borders, muted states, dividers, overlays, and security/status messaging.
- Use a quiet professional visual language: high contrast, low noise, precise spacing, compact information density, and strong keyboard focus.
- Prefer system UI or an app-local sans stack for product UI. Do not use decorative serif/display typography for vault lists, forms, TOTP codes, or settings.
- All secret-bearing UI must default to masked or summarized states. Revealing, copying, exporting, deleting, or changing vault/security settings requires deliberate controls and clear feedback.
- Keep Neon Auth account UI visually distinct from vault unlock/master-password UI. Copy and layout should reinforce that account sign-in does not unlock secrets.

## Global Rules

### Color Palette

Use Tailwind color names in implementation wherever possible. Hex values are documented only to keep design references explicit.

| Role | Tailwind Token | Hex | CSS Variable |
|------|----------------|-----|--------------|
| Primary / selected action | `sky-600` | `#0284C7` | `--color-primary` |
| Primary hover / active | `sky-700` | `#0369A1` | `--color-primary-hover` |
| Primary subtle surface | `sky-50` | `#F0F9FF` | `--color-primary-subtle` |
| Primary subtle border | `sky-200` | `#BAE6FD` | `--color-primary-border` |
| Focus ring | `sky-500` | `#0EA5E9` | `--color-ring` |
| App background, light | `zinc-50` | `#FAFAFA` | `--color-background` |
| App background, dark | `zinc-950` | `#09090B` | `--color-background-dark` |
| Surface, light | `zinc-100` | `#F4F4F5` | `--color-surface` |
| Surface, dark | `zinc-900` | `#18181B` | `--color-surface-dark` |
| Border / input, light | `zinc-200` | `#E4E4E7` | `--color-border` |
| Border / input, dark | `zinc-800` | `#27272A` | `--color-border-dark` |
| Text, light | `zinc-950` | `#09090B` | `--color-text` |
| Text, dark | `zinc-50` | `#FAFAFA` | `--color-text-dark` |
| Muted text, light | `zinc-600` | `#52525B` | `--color-muted-text` |
| Muted text, dark | `zinc-400` | `#A1A1AA` | `--color-muted-text-dark` |

**Color Notes:** Tailwind Sky is reserved for primary actions, current selection, focus, and controlled emphasis. Tailwind Zinc carries every other UI role. Do not use Slate, Gray, Neutral, Amber, Green, Red, Purple, or custom chromatic ramps in product UI.

### Security and Status Color Rules

- Warning, error, success, locked, unlocked, copied, expired, and destructive states still use Zinc surfaces/text/borders. Communicate severity with labels, icon shape, wording, confirmation steps, placement, and motion, not extra hue families.
- Sky can highlight the active control or focus target, but it must not imply that a destructive or failed-security state is safe.
- If a platform-native control exposes unavoidable system colors, wrap it in Zinc/Sky surrounding UI and do not mirror those colors elsewhere.

### Typography

- **Heading Font:** system sans
- **Body Font:** system sans
- **Mood:** calm, precise, secure, local-first, professional, high-legibility
- **Font Stack:** `Inter`, `ui-sans-serif`, `system-ui`, `-apple-system`, `BlinkMacSystemFont`, `"Segoe UI"`, `sans-serif`

**CSS Import:**
```css
/* Prefer app-local/system fonts for the desktop vault; no remote font import required. */
```

### Spacing Variables

| Token | Value | Usage |
|-------|-------|-------|
| `--space-xs` | `4px` / `0.25rem` | Tight gaps |
| `--space-sm` | `8px` / `0.5rem` | Icon gaps, inline spacing |
| `--space-md` | `16px` / `1rem` | Standard padding |
| `--space-lg` | `24px` / `1.5rem` | Section padding |
| `--space-xl` | `32px` / `2rem` | Large gaps |
| `--space-2xl` | `48px` / `3rem` | Section margins |
| `--space-3xl` | `64px` / `4rem` | Large app-shell or onboarding padding |

### Shadow Depths

| Level | Value | Usage |
|-------|-------|-------|
| `--shadow-sm` | `0 1px 2px rgba(9,9,11,0.05)` | Subtle lift |
| `--shadow-md` | `0 4px 6px rgba(9,9,11,0.1)` | Cards, buttons |
| `--shadow-lg` | `0 10px 15px rgba(9,9,11,0.1)` | Modals, dropdowns |
| `--shadow-xl` | `0 20px 25px rgba(9,9,11,0.15)` | Modals and critical overlays |

---

## Component Specs

### Buttons

```css
/* Primary Button */
.btn-primary {
  background: #0284C7; /* sky-600 */
  color: #FAFAFA; /* zinc-50 */
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  transition: all 200ms ease;
  cursor: pointer;
}

.btn-primary:hover {
  opacity: 0.9;
}

/* Secondary Button */
.btn-secondary {
  background: transparent;
  color: #18181B; /* zinc-900 */
  border: 1px solid #D4D4D8; /* zinc-300 */
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  transition: all 200ms ease;
  cursor: pointer;
}
```

### Cards

```css
.card {
  background: #F4F4F5; /* zinc-100 */
  border: 1px solid #E4E4E7; /* zinc-200 */
  border-radius: 8px;
  padding: 24px;
  box-shadow: var(--shadow-md);
  transition: box-shadow 200ms ease, border-color 200ms ease, background-color 200ms ease;
  cursor: pointer;
}

.card:hover {
  box-shadow: var(--shadow-lg);
}
```

### Inputs

```css
.input {
  padding: 12px 16px;
  border: 1px solid #D4D4D8; /* zinc-300 */
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 200ms ease;
}

.input:focus {
  border-color: #0EA5E9; /* sky-500 */
  outline: none;
  box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.24);
}
```

### Modals

```css
.modal-overlay {
  background: rgba(9, 9, 11, 0.5); /* zinc-950/50 */
  backdrop-filter: blur(4px);
}

.modal {
  background: #FAFAFA; /* zinc-50 */
  border-radius: 8px;
  padding: 32px;
  box-shadow: var(--shadow-xl);
  max-width: 500px;
  width: 90%;
}
```

---

## Style Guidelines

**Style:** Dark Mode (OLED)

**Keywords:** local-first desktop, secure vault, high contrast, quiet surfaces, low light, readable, keyboard-first, precise

**Best For:** Night-mode apps, coding platforms, entertainment, eye-strain prevention, OLED devices, low-light

**Key Effects:** Short color/opacity transitions, low bright-surface emission, high readability, visible focus

### Page Pattern

**Pattern Name:** Desktop Vault Work Surface

- **Primary Strategy:** Fast secure access to local secrets with deliberate reveal/copy/edit actions.
- **Primary Action Placement:** Contextual toolbar, item detail pane, command menu, and security dialogs.
- **Section Order:** 1. Unlock/account gate where applicable, 2. App shell, 3. Sidebar, 4. Search/command bar, 5. List or table, 6. Detail pane, 7. Status/toast region.

**Repo Override:** Public marketing pages are out of scope for the current design-system direction. In-app routes such as unlock, onboarding, vault, item detail, settings, and account screens must use desktop application layouts.

---

## Anti-Patterns (Do NOT Use)

- ❌ Theme that ignores OS preference or fails contrast in either light or dark mode
- ❌ Non-Sky chromatic UI colors such as Amber, Green, Red, Purple, Slate, Gray, or custom status ramps
- ❌ Slow rendering

### Additional Forbidden Patterns

- ❌ **Emojis as icons** — Use SVG icons (Heroicons, Lucide, Simple Icons)
- ❌ **Missing cursor:pointer** — All clickable elements must have cursor:pointer
- ❌ **Layout-shifting hovers** — Avoid scale transforms that shift layout
- ❌ **Low contrast text** — Maintain 4.5:1 minimum contrast ratio
- ❌ **Instant state changes** — Always use transitions (150-300ms)
- ❌ **Invisible focus states** — Focus states must be visible for a11y

---

## Pre-Delivery Checklist

Before delivering any UI code, verify:

- [ ] No emojis used as icons (use SVG instead)
- [ ] All icons from consistent icon set (Heroicons/Lucide)
- [ ] `cursor-pointer` on all clickable elements
- [ ] Hover states with smooth transitions (150-300ms)
- [ ] Light mode: text contrast 4.5:1 minimum
- [ ] Focus states visible for keyboard navigation
- [ ] `prefers-reduced-motion` respected
- [ ] Responsive: 375px, 768px, 1024px, 1440px
- [ ] No content hidden behind fixed navbars
- [ ] No horizontal scroll on mobile
