import { invoke } from '@tauri-apps/api/core';
import type {
  PasswordGeneratorOptions,
  VaultItemDetail,
  VaultItemSummary,
  VaultState
} from './types';

const browserVaultStateKey = 'laojie-river:vault-state';

function isTauriRuntime() {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

function readBrowserVaultState(): VaultState {
  if (typeof localStorage === 'undefined') {
    return { hasVault: false, unlocked: false };
  }

  const storedState = localStorage.getItem(browserVaultStateKey);
  if (!storedState) return { hasVault: false, unlocked: false };

  try {
    return JSON.parse(storedState) as VaultState;
  } catch {
    return { hasVault: false, unlocked: false };
  }
}

function writeBrowserVaultState(state: VaultState) {
  localStorage.setItem(browserVaultStateKey, JSON.stringify(state));
}

export const vaultApi = {
  getVaultState() {
    if (!isTauriRuntime()) return Promise.resolve(readBrowserVaultState());

    return invoke<VaultState>('get_vault_state');
  },
  createVault(masterPassword: string) {
    if (!isTauriRuntime()) {
      if (masterPassword.length < 12) {
        return Promise.reject(new Error('Use a longer master password or passphrase.'));
      }

      writeBrowserVaultState({ hasVault: true, unlocked: true });
      return Promise.resolve();
    }

    return invoke<void>('create_vault', { masterPassword });
  },
  unlockVault(masterPassword: string) {
    if (!isTauriRuntime()) {
      if (masterPassword.length === 0) {
        return Promise.reject(new Error('Master password is required.'));
      }

      writeBrowserVaultState({ hasVault: true, unlocked: true });
      return Promise.resolve();
    }

    return invoke<void>('unlock_vault', { masterPassword });
  },
  lockVault() {
    if (!isTauriRuntime()) {
      const state = readBrowserVaultState();
      writeBrowserVaultState({ ...state, unlocked: false });
      return Promise.resolve();
    }

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
