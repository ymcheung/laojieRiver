import { invoke } from '@tauri-apps/api/core';
import type {
  PasswordGeneratorOptions,
  VaultItemDetail,
  VaultItemSummary,
  VaultState
} from './types';

export const vaultApi = {
  getVaultState() {
    return invoke<VaultState>('get_vault_state');
  },
  createVault(masterPassword: string) {
    return invoke<void>('create_vault', { masterPassword });
  },
  unlockVault(masterPassword: string) {
    return invoke<void>('unlock_vault', { masterPassword });
  },
  lockVault() {
    return invoke<void>('lock_vault');
  },
  listItems() {
    return invoke<VaultItemSummary[]>('list_items');
  },
  getItem(id: string) {
    return invoke<VaultItemDetail>('get_item', { id });
  },
  copyPassword(id: string) {
    return invoke<void>('copy_password', { id });
  },
  generatePassword(options: PasswordGeneratorOptions) {
    return invoke<string>('generate_password', { options });
  }
};

