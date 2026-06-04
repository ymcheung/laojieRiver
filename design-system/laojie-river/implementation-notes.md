# LaoJie River Implementation Notes

These notes adapt the persisted `ui-ux-pro-max` search to the PRD for a SvelteKit + Tailwind + Bits UI + Tauri 2 password manager and TOTP vault.

## Product Constraints

- This is a cryptographic vault UI. The Svelte frontend renders summaries, forms, navigation, and safe command results; it must not become the source of truth for decrypted secrets.
- Keep service account identity and vault identity visually separate. Neon Auth screens can show account/session/device language; vault screens must center master-password, local unlock, auto-lock, and unrecoverable-secret expectations.
- Use secure defaults in the interface: masked password fields, concealed TOTP secrets, explicit reveal buttons, short-lived copy feedback, and no plaintext values in toasts, logs, empty states, or error text.

## Visual Direction

- Style: calm, precise, low-noise, professional, privacy-focused.
- Theme: Zinc-based light/dark with high contrast. Tailwind Sky is the only chromatic theme color and should be reserved for primary actions, selection, focus, and controlled emphasis. All other UI color must use Tailwind Zinc, including surfaces, text, borders, overlays, dividers, muted states, and security/status states.
- Security/status states: warning, error, success, destructive, locked, unlocked, copied, expired, and near-expiry states must not introduce amber, green, red, or other hue families. Use Zinc contrast, explicit labels, icon shape, confirmation flows, placement, and short motion to communicate severity.
- Typography: use a readable sans stack for most UI. Use tabular numerals for TOTP codes, countdowns, timestamps, and password metadata.
- Surfaces: app shell first. Use sidebars, split panes, tables/lists, detail panels, dialogs, popovers, and command menus. Avoid nested cards and marketing-style hero sections inside the unlocked app.
- Motion: use short 150-200ms opacity/color transitions. Avoid layout-moving hover transforms; respect reduced motion.

## SvelteKit, Bits UI, and Tailwind

- Build local wrapper components around Bits UI primitives so Button, Input, Dialog, AlertDialog, DropdownMenu, Tooltip, Popover, Command, Tabs, Switch, Checkbox, Badge, Toast, Sidebar, SearchBox, StrengthMeter, and TotpTimerRing share tokens.
- Use Tailwind tokens backed by CSS variables for background, foreground, card, muted, border, input, primary, ring, and radius. Map destructive, warning, and success semantics onto Zinc tokens plus labels and component structure rather than separate color families.
- Every icon-only button needs an accessible label and visible tooltip where the action is not obvious. Use one icon set consistently, preferably Lucide.
- Use `focus-visible` rings for all interactive elements, preserve dialog focus traps, support Escape where expected, and keep tab order aligned with the visual order.
- Use `aria-live="polite"` for non-secret status changes such as copied/cleared, synced/offline, vault locked, invalid TOTP setup, and save success or failure.

## Dashboard Shape

- Layout: left navigation, top command/search bar, item list, and detail panel. The list should work well at desktop widths and collapse predictably for narrower windows.
- Navigation groups: All Items, Favorites, Logins, Secure Notes, Cards, Identities, 2FA, Trash, Settings.
- Primary workflow: search with Cmd/Ctrl+K, add item, copy field, reveal field, edit, lock vault.
- TOTP UI: show large tabular 6-digit code, issuer/account label, circular or linear countdown, copy action, and clear near-expiry state using Zinc contrast, labels, and progress treatment. Never show the raw secret after setup unless the user deliberately enters an edit/reveal flow.
- Empty states should be direct and operational, such as "No passwords yet" or "No 2FA codes yet"; do not use playful illustrations or novelty copy.

## Security UX

- Destructive actions use AlertDialog with exact object names and consequences.
- Copy actions must confirm without exposing copied content and should align with Rust clipboard clearing behavior.
- Lock state should be immediately visible in the app shell. After auto-lock, clear sensitive detail panes and route to unlock without implying account sign-out.
- Error messages should be safe public errors: incorrect master password, vault is locked, invalid backup file, invalid 2FA secret, could not copy to clipboard.
