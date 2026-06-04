export type VaultState = {
  hasVault: boolean;
  unlocked: boolean;
};

export type VaultItemKind = 'login' | 'secure_note' | 'card' | 'identity';

export type VaultItemSummary = {
  id: string;
  kind: VaultItemKind;
  title: string;
  username?: string;
  url?: string;
  favorite: boolean;
  updatedAt: string;
};

export type TotpConfig = {
  secret: string;
  issuer?: string;
  accountName?: string;
  algorithm: 'SHA1' | 'SHA256' | 'SHA512';
  digits: 6 | 8;
  period: 30 | 60;
};

export type VaultItemDetail = VaultItemSummary & {
  password?: string;
  notes?: string;
  tags: string[];
  totp?: TotpConfig;
  createdAt: string;
};

export type PasswordGeneratorOptions = {
  length: number;
  uppercase: boolean;
  lowercase: boolean;
  numbers: boolean;
  symbols: boolean;
  avoidAmbiguous: boolean;
};

