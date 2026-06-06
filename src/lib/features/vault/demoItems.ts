import type { VaultItemSummary } from './types';

export const demoVaultItems: VaultItemSummary[] = [
  {
    id: 'sample-1',
    kind: 'login',
    title: 'GitHub',
    username: 'security@example.com',
    favorite: true,
    updatedAt: 'Today'
  },
  {
    id: 'sample-2',
    kind: 'login',
    title: 'Neon',
    username: 'sync@example.com',
    favorite: false,
    updatedAt: 'Yesterday'
  },
  {
    id: 'sample-3',
    kind: 'secure_note',
    title: 'Recovery codes',
    favorite: false,
    updatedAt: 'Last week'
  },
  {
    id: 'sample-4',
    kind: 'card',
    title: 'Travel card',
    favorite: false,
    updatedAt: 'May 28'
  },
  {
    id: 'sample-5',
    kind: 'identity',
    title: 'Personal identity',
    favorite: false,
    updatedAt: 'May 18'
  }
];
