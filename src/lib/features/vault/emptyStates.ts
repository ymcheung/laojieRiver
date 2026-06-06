export type EmptyStateIcon = 'key' | 'star' | 'note' | 'card' | 'identity' | 'totp';

export type EmptyStateContent = {
  eyebrow: string;
  title: string;
  description: string;
  primaryAction: string;
  secondaryAction?: string;
  icon: EmptyStateIcon;
};

const emptyStates: Record<string, EmptyStateContent> = {
  all: {
    eyebrow: 'Vault is ready',
    title: 'No items in this vault yet',
    description:
      'Create a login, secure note, card, identity, or 2FA entry after the Rust vault storage layer is connected.',
    primaryAction: 'New item',
    secondaryAction: 'Import backup',
    icon: 'key'
  },
  favorites: {
    eyebrow: 'Pinned items',
    title: 'No favorites yet',
    description:
      'Mark high-priority items as favorites so they stay easy to reach without exposing secret details in the list.',
    primaryAction: 'Browse all items',
    icon: 'star'
  },
  logins: {
    eyebrow: 'Passwords',
    title: 'No login items yet',
    description:
      'Add your first login once encrypted item creation is available. Password generation should stay on the Rust side.',
    primaryAction: 'Add login',
    icon: 'key'
  },
  'secure-notes': {
    eyebrow: 'Encrypted notes',
    title: 'No secure notes yet',
    description:
      'Use secure notes for recovery codes or private text that should be encrypted with the vault key.',
    primaryAction: 'Add secure note',
    icon: 'note'
  },
  cards: {
    eyebrow: 'Payment cards',
    title: 'No cards saved yet',
    description:
      'Store card details only after encrypted storage, field masking, and explicit reveal controls are implemented.',
    primaryAction: 'Add card',
    icon: 'card'
  },
  identities: {
    eyebrow: 'Profiles',
    title: 'No identities yet',
    description:
      'Identity items can hold profile details for form filling later. Keep summaries minimal while the vault is locked.',
    primaryAction: 'Add identity',
    icon: 'identity'
  },
  '2fa': {
    eyebrow: 'TOTP',
    title: 'No 2FA codes yet',
    description:
      'Paste an otpauth URL or add a secret manually once TOTP parsing is implemented in Rust.',
    primaryAction: 'Add 2FA code',
    secondaryAction: 'Paste otpauth URL',
    icon: 'totp'
  }
};

export function getVaultEmptyState(section: string) {
  return emptyStates[section] ?? emptyStates.all;
}
