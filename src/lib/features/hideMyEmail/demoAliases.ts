import type { HideMyEmailAliasSummary } from './types';

export const demoHideMyEmailAliases: HideMyEmailAliasSummary[] = [
  {
    id: 'demo-hme-1',
    address: 'quiet.river@icloud.com',
    label: 'GitHub',
    origin: 'github.com',
    isActive: true,
    updatedAt: 'Today'
  },
  {
    id: 'demo-hme-2',
    address: 'hidden.moon@icloud.com',
    label: 'Newsletter',
    origin: 'example.com',
    isActive: true,
    updatedAt: 'Yesterday'
  },
  {
    id: 'demo-hme-3',
    address: 'misty.garden@icloud.com',
    label: 'Old trial',
    isActive: false,
    updatedAt: 'May 18'
  }
];
