export type SettingsSection = {
  id: string;
  label: string;
  description: string;
  icon: 'security' | 'appearance' | 'backup' | 'import-export' | 'clipboard' | 'auto-lock' | 'demo';
};

export const settingsSections: SettingsSection[] = [
  {
    id: 'demo',
    label: 'Demo mode',
    description: 'Sample vault access for product review without a Neon Auth account.',
    icon: 'demo'
  },
  {
    id: 'security',
    label: 'Security',
    description: 'Master password, vault lock behavior, and recovery limits.',
    icon: 'security'
  },
  {
    id: 'appearance',
    label: 'Appearance',
    description: 'Theme, density, and window preferences.',
    icon: 'appearance'
  },
  {
    id: 'backup',
    label: 'Backup',
    description: 'Encrypted backup destinations and restore checks.',
    icon: 'backup'
  },
  {
    id: 'import-export',
    label: 'Import / Export',
    description: 'CSV, encrypted archive, and plaintext export warnings.',
    icon: 'import-export'
  },
  {
    id: 'clipboard',
    label: 'Clipboard',
    description: 'Copy timeout and clear-after-copy behavior.',
    icon: 'clipboard'
  },
  {
    id: 'auto-lock',
    label: 'Auto-lock',
    description: 'Inactivity, sleep, background, and restart locking.',
    icon: 'auto-lock'
  }
];
