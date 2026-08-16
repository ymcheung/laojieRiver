import { invoke } from '@tauri-apps/api/core';
import type {
  PasswordGeneratorOptions,
  VaultItemDetail,
  VaultItemSummary,
  VaultState
} from './types';

function isTauriRuntime() {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

function requireTauri() {
  return Promise.reject(new Error('Real vault operations require the Tauri desktop app.'));
}

export const vaultApi = {
  getVaultState() {
    if (!isTauriRuntime()) return Promise.resolve({ hasVault: false, unlocked: false });

    return invoke<VaultState>('get_vault_state');
  },
  createVault(masterPassword: string) {
    if (!isTauriRuntime()) return requireTauri();

    return invoke<void>('create_vault', { masterPassword });
  },
  unlockVault(masterPassword: string) {
    if (!isTauriRuntime()) return requireTauri();

    return invoke<void>('unlock_vault', { masterPassword });
  },
  lockVault() {
    if (!isTauriRuntime()) return requireTauri();

    return invoke<void>('lock_vault');
  },
  discardVault() {
    if (!isTauriRuntime()) return Promise.resolve();

    return invoke<void>('discard_vault');
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
