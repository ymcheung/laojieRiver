# Password Manager + 2FA Desktop App Plan

## 1. Product Goal

Build a secure, local-first desktop app for:

- Password management
- TOTP-based 2FA code generation
- Secure encrypted vault storage
- macOS and Windows support
- Clean, accessible UI using **Bits UI**
- Custom visual design using **Tailwind CSS**
- Native desktop integration using **Tauri 2**
- Security-sensitive logic implemented in **Rust**

## 2. Core Principle

This app should not be treated as a normal CRUD app.

It should be treated as a **cryptographic vault application**.

The frontend should never be the source of truth for secrets. The Rust backend should own:

- Vault unlock
- Encryption and decryption
- Password generation
- TOTP generation
- Clipboard control
- Auto-lock behavior
- File storage
- Import/export
- OS keychain integration

The Svelte frontend should mainly handle:

- UI rendering
- Form input
- Navigation
- User interaction
- Invoking safe Tauri commands

---

# 2.1 User Management and Neon Auth

## 2.1.1 Important Product Distinction

There are two different concepts in this product:

```txt
App user account = cloud identity, subscription, device sync, account settings
Vault identity = master password, vault key, encrypted local secrets
```

For a password manager, these must stay separate.

**Neon Auth should manage the service account.**

It should be used for:

- Sign up
- Sign in
- Email verification
- Password reset for the app account
- OAuth login, if desired
- Subscription/account identity
- Device registration
- Sync ownership
- Row Level Security identity in Neon Postgres

It should **not** be used for:

- The vault master password
- The vault encryption key
- Recovering encrypted passwords
- Decrypting user secrets on the server
- Storing plaintext passwords or TOTP secrets

The app account password and the vault master password should be treated as separate credentials.

## 2.1.2 Recommended Model

Use this model:

```txt
Neon Auth account
  └─ identifies user for cloud service

Local vault master password
  └─ derives key-encryption-key
      └─ decrypts local vault key
          └─ decrypts passwords and TOTP secrets
```

If cloud sync is added:

```txt
Desktop app
  ├─ logs in with Neon Auth
  ├─ receives authenticated session/JWT
  ├─ encrypts vault data locally before upload
  └─ syncs encrypted blobs to Neon Postgres

Neon Postgres
  ├─ stores users/sessions through Neon Auth
  ├─ stores encrypted vault blobs
  ├─ enforces ownership with RLS
  └─ never sees plaintext passwords, TOTP secrets, master password, or vault key
```

## 2.1.3 Neon Auth Role in the Architecture

Neon Auth should sit on the service side:

```txt
┌─────────────────────────────────────────┐
│ Tauri Desktop App                        │
│ - SvelteKit UI                           │
│ - Bits UI components                     │
│ - Tailwind styling                       │
│ - Rust vault core                        │
│ - Local encryption/decryption            │
└─────────────────────┬───────────────────┘
                      │ Authenticated API / Data API calls
                      ▼
┌─────────────────────────────────────────┐
│ Neon Auth                                │
│ - User identity                          │
│ - Sessions                               │
│ - OAuth/email auth                       │
│ - Password reset for service account     │
└─────────────────────┬───────────────────┘
                      ▼
┌─────────────────────────────────────────┐
│ Neon Postgres                            │
│ - neon_auth schema                       │
│ - app_user_profiles                      │
│ - devices                                │
│ - encrypted_vault_items                  │
│ - sync metadata                          │
│ - RLS policies                           │
└─────────────────────────────────────────┘
```

## 2.1.4 Recommended User Tables

Neon Auth stores auth-owned data in the `neon_auth` schema. Your app should create its own app tables and reference the authenticated user id.

Example app-level schema:

```sql
CREATE TABLE app_user_profiles (
  user_id UUID PRIMARY KEY,
  display_name TEXT,
  plan TEXT NOT NULL DEFAULT 'free',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE devices (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  device_name TEXT NOT NULL,
  platform TEXT NOT NULL CHECK (platform IN ('macos', 'windows')),
  public_key TEXT,
  last_seen_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE vaults (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  name TEXT NOT NULL DEFAULT 'Personal Vault',
  encrypted_vault_key BYTEA NOT NULL,
  kdf TEXT NOT NULL,
  kdf_params JSONB NOT NULL,
  salt BYTEA NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE encrypted_vault_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  vault_id UUID NOT NULL REFERENCES vaults(id) ON DELETE CASCADE,
  kind TEXT NOT NULL,
  encrypted_blob BYTEA NOT NULL,
  nonce BYTEA NOT NULL,
  client_updated_at TIMESTAMPTZ NOT NULL,
  server_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at TIMESTAMPTZ
);
```

Notes:

- `encrypted_blob` contains the encrypted login, note, password, and TOTP secret.
- `kind` leaks a small amount of metadata. You can remove it or encrypt it later if you want stronger privacy.
- `client_updated_at` is useful for offline-first conflict resolution.
- `deleted_at` supports soft deletes and cross-device sync.

## 2.1.5 RLS Direction

When using Neon Auth with Postgres, use RLS so users can only access their own rows.

Conceptually:

```sql
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
ALTER TABLE encrypted_vault_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE devices ENABLE ROW LEVEL SECURITY;

CREATE POLICY vaults_owner_select
ON vaults
FOR SELECT
USING (user_id = auth.user_id());

CREATE POLICY vault_items_owner_select
ON encrypted_vault_items
FOR SELECT
USING (user_id = auth.user_id());
```

The exact helper function name may differ depending on the Neon Auth SDK/RLS integration you use, so confirm against the current Neon Auth docs when implementing.

## 2.1.6 Auth Flow for a Tauri Desktop App

Recommended flow:

```txt
1. User opens desktop app
2. App checks whether local vault exists
3. User signs in to service account with Neon Auth
4. App receives session securely
5. App unlocks local vault with master password
6. App syncs encrypted vault blobs if sync is enabled
```

For desktop, avoid embedding a full remote website inside the app if possible. Prefer:

```txt
System browser login
  → custom URI callback to Tauri app
  → store session safely
  → app continues locally
```

Alternative for MVP:

```txt
Email/password form inside Tauri UI
  → call Neon Auth API/SDK
  → store session token securely
```

## 2.1.7 Session Storage

Store Neon Auth session material using OS-secure storage:

```txt
macOS: Keychain
Windows: Credential Manager
```

Do not store long-lived session tokens in:

```txt
localStorage
IndexedDB
plain SQLite
plain JSON config files
frontend Svelte stores after app lock
```

The vault lock and service auth state should be separate:

```txt
Signed in but vault locked
  = user identity is known, but secrets are unavailable

Signed out but local vault exists
  = user can still unlock local-only vault if offline mode is allowed
```

## 2.1.8 Account Password Reset vs Vault Recovery

This is critical.

Neon Auth can support password reset for the **service account**.

It cannot safely reset the **vault master password** unless you build a recovery-key system.

Recommended UX copy:

```txt
Your account password lets you sign in to sync and manage your subscription.
Your vault master password unlocks your encrypted passwords.
We cannot recover your vault master password.
```

## 2.1.9 Cloud Sync Data Flow

For sync, use zero-knowledge style encryption:

```txt
Create/update item
  → encrypt locally in Rust
  → upload encrypted_blob + nonce + metadata
  → Neon stores encrypted data
  → another device downloads encrypted data
  → decrypts locally after vault unlock
```

The server should never receive:

```txt
plaintext passwords
plaintext notes
plaintext TOTP secrets
master password
vault key
key-encryption-key
```

## 2.1.10 Suggested Auth-Related MVP Scope

For MVP, add:

```txt
Sign up
Sign in
Sign out
Email verification
Password reset for service account
Local vault create/unlock
Optional encrypted sync toggle
Device list
Delete device
Delete account
Export encrypted backup before account deletion
```

Do not add yet:

```txt
Team vaults
Organization sharing
Admin console
Emergency access
Shared folders
Family plans
Enterprise SSO
SCIM
```

## 2.1.11 SvelteKit + Bits UI Auth Screens

Build your own UI with Bits UI and Tailwind instead of relying on prebuilt React components.

Recommended screens:

```txt
/auth/sign-in
/auth/sign-up
/auth/verify-email
/auth/forgot-password
/auth/reset-password
/account
/account/devices
/account/security
```

Bits UI components to use:

```txt
Dialog: confirm sign out, delete account, delete device
Tabs: account settings sections
DropdownMenu: account menu
Tooltip: explain vault vs account password
AlertDialog: destructive actions
Switch: enable/disable sync
Input: email/password fields
```

## 2.1.12 Product Rule

The cleanest product rule is:

```txt
Neon Auth protects the service account.
The master password protects the vault.
The server stores encrypted data only.
The desktop app decrypts locally only after vault unlock.
```

---

# 3. Recommended Tech Stack

## 3.1 Desktop Runtime

Use:

```txt
Tauri 2
```

Reasons:

- Smaller app size than Electron
- Rust backend
- Good fit for security-sensitive local applications
- Cross-platform macOS and Windows support
- Permission/capability-based security model
- Web frontend without giving the webview unlimited native access

## 3.2 Frontend

Use:

```txt
SvelteKit
Bits UI
Tailwind CSS
TypeScript
```

Reasons:

- Good fit for Tauri-based desktop apps
- Bits UI is headless, accessible, and fully styleable
- Tailwind is ideal for fast, consistent styling
- SvelteKit gives structured routing and app organization

## 3.3 Backend/Core

Use:

```txt
Rust
SQLite
Argon2id
XChaCha20-Poly1305 or AES-256-GCM
HMAC-based TOTP
```

The Rust side should be the security boundary.

## 3.4 Storage

Use:

```txt
SQLite with encrypted blobs
```

Do not store passwords as plain rows like this:

```sql
username TEXT,
password TEXT,
totp_secret TEXT
```

Instead, store encrypted blobs:

```sql
id TEXT PRIMARY KEY,
kind TEXT,
encrypted_blob BLOB,
nonce BLOB,
created_at TEXT,
updated_at TEXT
```

---

# 4. High-Level Architecture

```txt
┌─────────────────────────────────────────┐
│ SvelteKit Frontend                       │
│ - Bits UI components                     │
│ - Tailwind styling                       │
│ - Forms, lists, dialogs                  │
│ - No direct secret storage               │
└─────────────────────┬───────────────────┘
                      │ Tauri invoke()
                      ▼
┌─────────────────────────────────────────┐
│ Tauri Command Layer                      │
│ - Typed command interface                │
│ - Permission-limited capabilities        │
│ - Input validation                       │
└─────────────────────┬───────────────────┘
                      ▼
┌─────────────────────────────────────────┐
│ Rust Vault Core                          │
│ - Vault unlock/lock                      │
│ - Encryption/decryption                  │
│ - Password generator                     │
│ - TOTP generator                         │
│ - Clipboard manager                      │
│ - Auto-lock manager                      │
└─────────────────────┬───────────────────┘
                      ▼
┌─────────────────────────────────────────┐
│ Local Storage                            │
│ - SQLite encrypted blobs                 │
│ - Vault metadata                         │
│ - Encrypted backup files                 │
└─────────────────────────────────────────┘
```

---

# 5. Security Model

## 5.1 Threat Model

Define what the app protects against.

### In Scope

The app should protect against:

- Someone stealing the vault file
- Someone accessing the disk while the vault is locked
- App logs accidentally exposing secrets
- Basic malware reading app config files
- Clipboard leakage after a timeout
- Unauthorized UI access after inactivity
- Cloud/server compromise, if sync is added later

### Out of Scope

The app probably cannot fully protect against:

- Active malware controlling the user’s machine
- Keyloggers
- Screen recorders
- Compromised OS
- Malicious browser extensions reading copied passwords
- A user choosing a weak master password

Be honest about this in the product.

---

# 6. Vault Design

## 6.1 Vault Creation Flow

```txt
User creates master password
        ↓
Generate random salt
        ↓
Derive key-encryption-key using Argon2id
        ↓
Generate random vault key
        ↓
Encrypt vault key with key-encryption-key
        ↓
Store encrypted vault key + KDF parameters
```

## 6.2 Unlock Flow

```txt
User enters master password
        ↓
Load vault metadata
        ↓
Derive key-encryption-key using stored Argon2id params
        ↓
Decrypt vault key
        ↓
Keep vault key in memory only
        ↓
App enters unlocked state
```

## 6.3 Item Encryption Flow

```txt
Login item JSON
        ↓
Serialize to canonical format
        ↓
Generate nonce
        ↓
Encrypt with vault key
        ↓
Store encrypted blob in SQLite
```

## 6.4 Recommended Vault Metadata

```json
{
  "vault_version": 1,
  "kdf": "argon2id",
  "kdf_params": {
    "memory_cost": 194560,
    "time_cost": 2,
    "parallelism": 1
  },
  "salt": "...",
  "encrypted_vault_key": "...",
  "created_at": "...",
  "updated_at": "..."
}
```

---

# 7. Data Model

## 7.1 Decrypted Item Shape

```ts
type VaultItem = {
  id: string;
  kind: "login" | "secure_note" | "card" | "identity";
  title: string;
  username?: string;
  password?: string;
  url?: string;
  notes?: string;
  totp?: TotpConfig;
  tags: string[];
  favorite: boolean;
  createdAt: string;
  updatedAt: string;
};
```

## 7.2 TOTP Config

```ts
type TotpConfig = {
  secret: string;
  issuer?: string;
  accountName?: string;
  algorithm: "SHA1" | "SHA256" | "SHA512";
  digits: 6 | 8;
  period: 30 | 60;
};
```

## 7.3 SQLite Tables

```sql
CREATE TABLE vault_metadata (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  vault_version INTEGER NOT NULL,
  kdf TEXT NOT NULL,
  kdf_params TEXT NOT NULL,
  salt BLOB NOT NULL,
  encrypted_vault_key BLOB NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE vault_items (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  encrypted_blob BLOB NOT NULL,
  nonce BLOB NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE item_index (
  item_id TEXT PRIMARY KEY,
  title_hash TEXT,
  url_hash TEXT,
  kind TEXT NOT NULL,
  favorite INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT NOT NULL
);
```

Important: even `item_index` can leak metadata. Keep it minimal.

---

# 8. Frontend Architecture

## 8.1 Recommended Folder Structure

```txt
src/
  app.css
  app.html

  lib/
    components/
      ui/
        button/
        dialog/
        dropdown-menu/
        input/
        command/
        tooltip/
        tabs/
        switch/
        toast/

      vault/
        VaultLayout.svelte
        VaultSidebar.svelte
        VaultItemList.svelte
        VaultItemDetail.svelte
        VaultSearch.svelte
        PasswordField.svelte
        TotpCode.svelte
        StrengthMeter.svelte

      onboarding/
        CreateVaultForm.svelte
        UnlockVaultForm.svelte
        RecoveryKeyNotice.svelte

    features/
      vault/
        api.ts
        types.ts
        stores.svelte.ts

      settings/
        types.ts
        api.ts

      import-export/
        api.ts

    styles/
      tokens.css

    utils/
      cn.ts
      time.ts
      validation.ts

  routes/
    +layout.svelte
    +page.svelte
    unlock/
      +page.svelte
    vault/
      +layout.svelte
      +page.svelte
      [itemId]/
        +page.svelte
    settings/
      +page.svelte
```

## 8.2 UI State Philosophy

Avoid this:

```ts
let allPasswords = writable<DecryptedVaultItem[]>([]);
```

Prefer this:

```ts
let visibleItems = writable<VaultItemSummary[]>([]);
let selectedItem = writable<VaultItemDetail | null>(null);
```

The frontend should receive only what it needs.

---

# 9. Bits UI Usage Plan

Bits UI should be your primitive layer.

Use it for:

- Dialogs
- Dropdown menus
- Context menus
- Tabs
- Tooltips
- Popovers
- Selects
- Switches
- Checkboxes
- Radio groups
- Command/search interface
- Toasts, if available or via your own wrapper

## 9.1 Component Strategy

Create your own design-system wrappers around Bits UI.

Example structure:

```txt
src/lib/components/ui/dialog/
  Dialog.svelte
  DialogContent.svelte
  DialogHeader.svelte
  DialogTitle.svelte
  DialogDescription.svelte
  DialogFooter.svelte
```

This lets you keep Bits UI replaceable and gives your app consistent styling.

## 9.2 Example Button Component

```svelte
<script lang="ts">
  import { cn } from "$lib/utils/cn";

  let {
    variant = "default",
    size = "md",
    class: className,
    ...rest
  } = $props();
</script>

<button
  class={cn(
    "inline-flex items-center justify-center rounded-lg font-medium transition",
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
    "disabled:pointer-events-none disabled:opacity-50",
    variant === "default" && "bg-primary text-primary-foreground hover:bg-primary/90",
    variant === "ghost" && "hover:bg-muted",
    variant === "danger" && "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    size === "sm" && "h-8 px-3 text-sm",
    size === "md" && "h-10 px-4 text-sm",
    size === "lg" && "h-11 px-6 text-base",
    className
  )}
  {...rest}
>
  <slot />
</button>
```

## 9.3 UI Components Needed

```txt
Button
Input
PasswordInput
Textarea
Dialog
AlertDialog
DropdownMenu
ContextMenu
Tooltip
Popover
Command
Tabs
Switch
Checkbox
Badge
Card
Toast
Sidebar
SearchBox
StrengthMeter
TotpTimerRing
```

---

# 10. Tailwind Design System

## 10.1 Tailwind Setup

Install:

```bash
npm install tailwindcss @tailwindcss/vite
```

In `vite.config.ts`:

```ts
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()]
});
```

In `src/app.css`:

```css
@import "tailwindcss";
```

In `src/routes/+layout.svelte`:

```svelte
<script lang="ts">
  import "../app.css";
</script>

<slot />
```

## 10.2 Design Direction

For a password manager, use a style that is:

```txt
calm
precise
low-noise
high-contrast
professional
privacy-focused
```

Avoid making it too playful.

## 10.3 Theme Tokens

Use CSS variables so Bits UI wrappers and Tailwind classes share the same design language.

```css
@import "tailwindcss";

:root {
  --background: 250 250 250;
  --foreground: 24 24 27;

  --muted: 244 244 245;
  --muted-foreground: 113 113 122;

  --card: 255 255 255;
  --card-foreground: 24 24 27;

  --border: 228 228 231;
  --input: 228 228 231;

  --primary: 24 24 27;
  --primary-foreground: 250 250 250;

  --destructive: 220 38 38;
  --destructive-foreground: 255 255 255;

  --ring: 39 39 42;

  --radius: 0.75rem;
}

.dark {
  --background: 9 9 11;
  --foreground: 250 250 250;

  --muted: 39 39 42;
  --muted-foreground: 161 161 170;

  --card: 24 24 27;
  --card-foreground: 250 250 250;

  --border: 63 63 70;
  --input: 63 63 70;

  --primary: 250 250 250;
  --primary-foreground: 24 24 27;

  --destructive: 239 68 68;
  --destructive-foreground: 255 255 255;

  --ring: 212 212 216;
}
```

---

# 11. Main App Screens

## 11.1 Onboarding

### Screens

```txt
Welcome
Create Vault
Confirm Master Password
Recovery Notice
Vault Created
```

### Key UX Requirements

- Explain that the master password cannot be recovered
- Encourage a long passphrase
- Show password strength
- Require confirmation
- Offer encrypted backup setup

## 11.2 Unlock Screen

Features:

- Master password input
- Optional biometric/OS unlock later
- Clear error messages
- Lockout delay after repeated failed attempts
- Keyboard-first interaction

## 11.3 Vault Home

Layout:

```txt
┌──────────────────────────────────────────────┐
│ Top Bar: Search / Add / Lock                 │
├───────────────┬──────────────────────────────┤
│ Sidebar       │ Item List                     │
│ - All Items   │ - GitHub                      │
│ - Favorites   │ - Apple ID                    │
│ - Logins      │ - Stripe                      │
│ - Notes       │                              │
│ - Tags        │                              │
├───────────────┴──────────────────────────────┤
│ Detail Panel                                 │
└──────────────────────────────────────────────┘
```

## 11.4 Item Detail

Fields:

```txt
Title
Username
Password
URL
TOTP
Notes
Tags
Created At
Updated At
```

Actions:

```txt
Copy username
Copy password
Reveal password
Generate new password
Copy TOTP
Edit
Duplicate
Delete
Favorite
Open URL in browser
```

## 11.5 Password Generator

Options:

```txt
Length
Uppercase
Lowercase
Numbers
Symbols
Avoid ambiguous characters
Passphrase mode
Word count
Separator
```

## 11.6 TOTP Setup

Methods:

```txt
Manual secret input
otpauth:// URL paste
QR code import from image
```

TOTP display:

```txt
Current code
Countdown ring
Copy button
Issuer
Account name
```

## 11.7 Settings

Sections:

```txt
Security
Appearance
Backup
Import / Export
Clipboard
Auto-lock
About
```

---

# 12. Rust Core Modules

Recommended structure:

```txt
src-tauri/
  src/
    main.rs

    commands/
      mod.rs
      vault_commands.rs
      item_commands.rs
      clipboard_commands.rs
      settings_commands.rs
      import_export_commands.rs

    core/
      mod.rs
      vault.rs
      crypto.rs
      storage.rs
      password_generator.rs
      totp.rs
      session.rs
      errors.rs

    platform/
      mod.rs
      keychain.rs
      macos.rs
      windows.rs
```

## 12.1 Rust Command API

Expose commands like:

```rust
#[tauri::command]
async fn create_vault(master_password: String) -> Result<(), AppError>;

#[tauri::command]
async fn unlock_vault(master_password: String) -> Result<(), AppError>;

#[tauri::command]
async fn lock_vault() -> Result<(), AppError>;

#[tauri::command]
async fn list_items() -> Result<Vec<VaultItemSummary>, AppError>;

#[tauri::command]
async fn get_item(id: String) -> Result<VaultItemDetail, AppError>;

#[tauri::command]
async fn create_item(input: CreateItemInput) -> Result<String, AppError>;

#[tauri::command]
async fn update_item(id: String, input: UpdateItemInput) -> Result<(), AppError>;

#[tauri::command]
async fn delete_item(id: String) -> Result<(), AppError>;

#[tauri::command]
async fn copy_password(id: String) -> Result<(), AppError>;

#[tauri::command]
async fn generate_totp(id: String) -> Result<TotpCode, AppError>;
```

## 12.2 Important Rule

Do not expose commands like:

```rust
#[tauri::command]
async fn decrypt_everything() -> Vec<PlaintextItem>;
```

That would make the frontend too powerful.

---

# 13. Tauri Security Plan

## 13.1 Capabilities

Use separate capabilities for different windows if needed.

Example windows:

```txt
main
unlock
settings
about
```

Give each window only what it needs.

For example:

```txt
unlock window:
- unlock vault
- create vault
- quit app

main window:
- list item summaries
- get selected item
- copy selected secret
- lock vault

settings window:
- update settings
- export encrypted backup
```

## 13.2 Disable Dangerous Patterns

Avoid:

```txt
loading remote web content
eval
inline scripts
unrestricted filesystem access
frontend-side crypto
storing secrets in localStorage
storing secrets in IndexedDB
logging command payloads
```

## 13.3 Content Security Policy

Use a strict CSP.

Example direction:

```txt
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data:;
connect-src 'self';
```

Be careful with `unsafe-inline`. Some frontend tooling may require it during development, but production should be stricter where possible.

---

# 14. Clipboard Handling

## 14.1 Requirements

When the user copies a password or TOTP code:

```txt
1. Copy secret to clipboard
2. Start timer
3. Clear after 30–60 seconds
4. Clear only if clipboard still contains the copied value
5. Do not log clipboard content
```

## 14.2 UX

Show toast:

```txt
Password copied. Clipboard will clear in 45 seconds.
```

For TOTP:

```txt
2FA code copied. Clipboard will clear soon.
```

---

# 15. Auto-Lock Plan

Auto-lock should trigger on:

```txt
App inactivity
System sleep
Screen lock
App backgrounded
Manual lock
Configurable timeout
```

Recommended defaults:

```txt
Auto-lock after inactivity: 5 minutes
Clear clipboard: 45 seconds
Require master password after app restart
Require master password after system sleep
```

---

# 16. Password Generator

## 16.1 Random Password Mode

Options:

```txt
Length: 8–128
Uppercase
Lowercase
Numbers
Symbols
Avoid ambiguous characters
Require every selected character group
```

## 16.2 Passphrase Mode

Options:

```txt
Word count: 4–10
Separator: -, _, ., space
Capitalization
Append number
```

## 16.3 Security

Use OS randomness from Rust.

Do not use:

```ts
Math.random()
```

for password generation.

---

# 17. TOTP / 2FA Plan

## 17.1 Supported Format

Support standard otpauth URLs:

```txt
otpauth://totp/Issuer:Account?secret=BASE32SECRET&issuer=Issuer&algorithm=SHA1&digits=6&period=30
```

## 17.2 Supported Algorithms

```txt
SHA1
SHA256
SHA512
```

## 17.3 Supported Digits

```txt
6
8
```

## 17.4 Supported Periods

```txt
30 seconds
60 seconds
```

## 17.5 UX

Each TOTP item should show:

```txt
123 456
Refreshes in 18s
Copy
```

Use a visual countdown ring or progress bar.

## 17.6 Product Warning

Storing passwords and TOTP secrets in the same vault is convenient but weakens true two-factor separation.

Add a note:

```txt
For stronger security, store 2FA codes on a separate device.
```

---

# 18. Import / Export

## 18.1 MVP Import

Support:

```txt
CSV import
Bitwarden export import
1Password export import
Raw otpauth URL import
```

Start with CSV and your own encrypted format first.

## 18.2 Export Modes

```txt
Encrypted backup
Plain CSV export
Plain JSON export
```

Encrypted backup should be the default.

Plain export should require:

```txt
Master password confirmation
Warning dialog
Explicit file destination
No auto-open
No logs
```

## 18.3 Own Backup Format

Example:

```json
{
  "format": "yourapp-vault-backup",
  "version": 1,
  "created_at": "...",
  "kdf": "argon2id",
  "encrypted_payload": "..."
}
```

---

# 19. Search Design

There are two possible search models.

## 19.1 Simple MVP Search

After unlock, decrypt item summaries into memory:

```txt
title
username
url
tags
kind
```

Search only these summaries.

Pros:

- Simple
- Fast
- Good UX

Cons:

- Metadata exists in memory while unlocked

## 19.2 Stronger Privacy Search

Store only encrypted items and decrypt/search on demand.

Pros:

- Less metadata exposure

Cons:

- Slower
- More complex UX

Recommendation:

For MVP, use simple unlocked-session search, but keep the index minimal.

---

# 20. Routing Plan

```txt
/                 redirect based on vault state
/unlock           unlock vault
/onboarding       create vault
/vault            item list
/vault/[itemId]   item detail
/settings         settings
/settings/security
/settings/backup
/settings/import-export
```

In Tauri, routing is local. Do not treat SvelteKit as a server app.

---

# 21. UI/UX Design Plan

## 21.1 Visual Style

Recommended:

```txt
Neutral dark/light theme
Minimal glass or paper-like surfaces
Strong focus states
Clear hierarchy
Keyboard-friendly
No visual noise
```

## 21.2 Main Navigation

Sidebar:

```txt
All Items
Favorites
Logins
Secure Notes
Cards
Identities
2FA
Trash
```

## 21.3 Search UX

Use a command-menu style search:

```txt
⌘K / Ctrl+K
Search items
Create new login
Generate password
Open settings
Lock vault
```

Bits UI is useful for this kind of composable headless interaction.

## 21.4 Empty States

Examples:

```txt
No passwords yet.
Create your first login item.
```

```txt
No 2FA codes yet.
Paste an otpauth URL or add a secret manually.
```

---

# 22. Accessibility

Minimum requirements:

```txt
Keyboard navigation
Visible focus rings
Dialog focus trap
Escape to close dialogs
Screen-reader-friendly labels
Reduced-motion support
Color contrast AA minimum
No icon-only buttons without labels
```

Bits UI gives accessible primitives, but your wrappers still need correct labels and interaction design.

---

# 23. Error Handling

## 23.1 User-Friendly Errors

Examples:

```txt
Incorrect master password.
Vault is locked.
This backup file is invalid.
This 2FA secret is not valid.
Could not copy to clipboard.
```

## 23.2 Internal Errors

Rust should map internal errors to safe public errors.

Do not expose:

```txt
stack traces
file paths with sensitive usernames
raw decrypted payloads
crypto internals
secret values
```

---

# 24. Testing Plan

## 24.1 Rust Unit Tests

Test:

```txt
vault creation
vault unlock
wrong password failure
item encryption/decryption
password generator
TOTP generation
backup export/import
clipboard clearing logic
```

## 24.2 Frontend Tests

Test:

```txt
unlock form validation
create item form
edit item flow
delete confirmation
copy button behavior
TOTP display behavior
settings forms
```

## 24.3 Integration Tests

Test:

```txt
create vault → add login → lock → unlock → read login
create TOTP item → generate code
export backup → import backup
wrong password cannot decrypt vault
```

## 24.4 Manual Security Tests

Check:

```txt
No secrets in logs
No secrets in devtools console
No secrets in localStorage
No secrets in IndexedDB
No plaintext vault file
No plaintext backup by default
Clipboard clears correctly
App locks after inactivity
```

---

# 25. Development Roadmap

## Phase 0: Research and Decisions

Deliverables:

```txt
Threat model
Vault format v1
Crypto design
App architecture diagram
UI wireframes
```

Decisions:

```txt
Local-only or sync later
Recovery model
Vault file location
Encrypted backup format
Supported import formats
```

## Phase 1: Project Setup

Tasks:

```txt
Create Tauri 2 + SvelteKit project
Add TypeScript
Add Tailwind CSS
Add Bits UI
Create app shell
Create Rust command structure
Set up SQLite
Set up logging policy
Set up error model
```

Install examples:

```bash
npm create tauri-app@latest
npm install bits-ui
npm install tailwindcss @tailwindcss/vite
```

## Phase 2: Design System

Tasks:

```txt
Create Tailwind tokens
Create Button
Create Input
Create Dialog wrappers
Create DropdownMenu wrappers
Create Tooltip wrappers
Create Tabs wrappers
Create Toast system
Create app layout
Create dark mode
```

## Phase 3: Vault MVP

Tasks:

```txt
Create vault
Unlock vault
Lock vault
Store vault metadata
Encrypt vault key
Decrypt vault key
Create encrypted item
Read encrypted item
Update encrypted item
Delete encrypted item
List item summaries
```

## Phase 4: Password Features

Tasks:

```txt
Add login item form
Add password reveal/hide
Add copy username
Add copy password
Add password generator
Add password strength meter
Add favorite/tag support
```

## Phase 5: 2FA Features

Tasks:

```txt
Add TOTP secret field
Parse otpauth URLs
Generate TOTP code
Add countdown UI
Add copy TOTP
Add issuer/account display
Add QR import later
```

## Phase 6: Security UX

Tasks:

```txt
Auto-lock
Clipboard clear
Lock on sleep
Lock on app background
Master password confirmation for export
Dangerous action confirmations
```

## Phase 7: Import / Export

Tasks:

```txt
Encrypted backup export
Encrypted backup import
CSV import
Plain CSV export with warning
Plain JSON export with warning
```

## Phase 8: Polish

Tasks:

```txt
Command menu
Keyboard shortcuts
Empty states
Loading states
Error states
Accessibility pass
Responsive desktop layouts
macOS polish
Windows polish
```

## Phase 9: Distribution

Tasks:

```txt
Code signing
macOS notarization
Windows signing
Auto-update strategy
Crash reporting without secrets
Release checklist
```

---

# 26. Security Checklist Before Release

```txt
[ ] No plaintext secrets in SQLite
[ ] No plaintext secrets in logs
[ ] No plaintext secrets in localStorage
[ ] No plaintext secrets in IndexedDB
[ ] No frontend-side vault decryption
[ ] No remote content loaded in app
[ ] Strict Tauri capabilities
[ ] Strict command input validation
[ ] Clipboard auto-clear implemented
[ ] Auto-lock implemented
[ ] Wrong password cannot decrypt vault
[ ] Backup export encrypted by default
[ ] Plain export requires warning + confirmation
[ ] App tested on macOS
[ ] App tested on Windows
[ ] Dependencies audited
[ ] Recovery limitations explained to user
```

---

# 27. Suggested MVP Scope

For the first serious version, build only this:

```txt
Create local vault
Unlock vault
Add/edit/delete login
Copy username/password
Generate password
Add/edit/delete TOTP secret
Generate TOTP code
Copy TOTP code
Auto-lock
Clipboard clear
Encrypted export/import
Dark/light theme
```

Do not build these yet:

```txt
Cloud sync
Browser extension
Team sharing
Mobile app
Passkey storage
Emergency access
Shared vaults
```

Those are much harder and increase security risk.

---

# 28. Recommended MVP Command List

Frontend calls Rust through a narrow command API:

```ts
export const vaultApi = {
  createVault(masterPassword: string): Promise<void>,
  unlockVault(masterPassword: string): Promise<void>,
  lockVault(): Promise<void>,
  getVaultState(): Promise<VaultState>,

  listItems(): Promise<VaultItemSummary[]>,
  getItem(id: string): Promise<VaultItemDetail>,
  createItem(input: CreateItemInput): Promise<string>,
  updateItem(id: string, input: UpdateItemInput): Promise<void>,
  deleteItem(id: string): Promise<void>,

  copyUsername(id: string): Promise<void>,
  copyPassword(id: string): Promise<void>,
  copyTotp(id: string): Promise<void>,

  generatePassword(options: PasswordGeneratorOptions): Promise<string>,
  previewTotp(input: TotpInput): Promise<TotpPreview>,

  exportEncryptedBackup(path: string): Promise<void>,
  importEncryptedBackup(path: string, password: string): Promise<void>
};
```

---

# 29. Final Recommended Direction

Use this stack:

```txt
Tauri 2
Rust
SvelteKit
Bits UI
Tailwind CSS
SQLite
Argon2id
XChaCha20-Poly1305 or AES-GCM
RFC 6238-compatible TOTP
macOS Keychain / Windows Credential Manager later
```

Build it in this order:

```txt
1. Vault crypto
2. Local encrypted storage
3. Unlock/lock lifecycle
4. Basic SvelteKit UI
5. Bits UI design system wrappers
6. Password item CRUD
7. Password generator
8. TOTP support
9. Clipboard + auto-lock
10. Import/export
11. Security review
12. Packaging
```

For this project, **Tauri + SvelteKit + Bits UI + Tailwind** is a strong combination: SvelteKit gives app structure, Bits UI gives accessible behavior, Tailwind gives complete visual control, and Rust keeps the sensitive parts away from the frontend.

---

# 30. Reference Links

- Tauri 2 Security: https://v2.tauri.app/security/
- Tauri 2 Capabilities: https://v2.tauri.app/security/capabilities/
- Bits UI Introduction: https://www.bits-ui.com/docs/introduction
- Bits UI Getting Started: https://www.bits-ui.com/docs/getting-started
- Tailwind CSS with SvelteKit: https://tailwindcss.com/docs/guides/sveltekit
- SvelteKit Docs: https://svelte.dev/docs/kit
- RFC 6238 TOTP: https://datatracker.ietf.org/doc/html/rfc6238
- OWASP Cryptographic Storage Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html
