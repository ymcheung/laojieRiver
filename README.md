# LaoJie River

Traditional Chinese title: 老街溪

LaoJie River is a local-first desktop vault for password management and TOTP-based 2FA. The project follows the PRD in [docs/PRD.md](docs/PRD.md): SvelteKit, Tailwind CSS, and Bits UI on the frontend, with Tauri 2 and Rust owning security-sensitive vault behavior.

## Current Scope

This repository is initialized for Phase 1:

- Tauri 2 desktop shell
- SvelteKit + TypeScript frontend
- Tailwind CSS token foundation
- Bits UI and Lucide dependencies for accessible, styleable UI
- Rust command/module skeleton for vault, item, clipboard, and settings surfaces
- PRD-backed route and component structure

The Rust vault commands are intentionally scaffold stubs. Production crypto, SQLite storage, clipboard clearing, auto-lock, and Neon Auth sync must be implemented before any real vault data is stored.

## Development

```bash
pnpm install
pnpm dev
```

Run the desktop app:

```bash
pnpm tauri dev
```

Check frontend types:

```bash
pnpm check
```

## Security Boundary

Frontend code must not become the source of truth for secrets. Use SvelteKit for rendering and interaction, then call narrow Tauri commands for:

- vault create/unlock/lock state
- item summaries and selected item details
- password generation
- TOTP generation
- clipboard operations
- import/export

Do not store plaintext secrets in `localStorage`, IndexedDB, logs, frontend global stores, or SQLite rows.
