export type VaultSection = {
  id: string;
  label: string;
  href: string;
  description: string;
};

export const vaultSections: VaultSection[] = [
  {
    id: 'all',
    label: 'All Items',
    href: '/vault',
    description: 'All unlocked item summaries'
  },
  {
    id: 'favorites',
    label: 'Favorites',
    href: '/vault?section=favorites',
    description: 'Pinned item summaries'
  },
  {
    id: 'logins',
    label: 'Logins',
    href: '/vault?section=logins',
    description: 'Password login items'
  },
  {
    id: 'secure-notes',
    label: 'Secure Notes',
    href: '/vault?section=secure-notes',
    description: 'Encrypted note items'
  },
  {
    id: 'cards',
    label: 'Cards',
    href: '/vault?section=cards',
    description: 'Payment card items'
  },
  {
    id: 'identities',
    label: 'Identities',
    href: '/vault?section=identities',
    description: 'Identity profile items'
  },
  {
    id: '2fa',
    label: '2FA',
    href: '/vault?section=2fa',
    description: 'Items with TOTP codes'
  }
];
