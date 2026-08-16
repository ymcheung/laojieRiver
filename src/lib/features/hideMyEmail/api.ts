import { invoke } from '@tauri-apps/api/core';
import type {
  HideMyEmailAliasSummary,
  HideMyEmailRefreshResult,
  HideMyEmailSetupResult,
  HideMyEmailStatus,
  HideMyEmailVerificationMethod
} from './types';

export const hideMyEmailApi = {
  status: () => invoke<HideMyEmailStatus>('get_hide_my_email_status'),
  listAliases: () => invoke<HideMyEmailAliasSummary[]>('list_hide_my_email_aliases'),
  startSetup: (appleId: string, password: string) =>
    invoke<HideMyEmailSetupResult>('start_hide_my_email_setup', {
      appleId,
      password,
      acknowledged: true
    }),
  verifySetup: (code: string, method: HideMyEmailVerificationMethod) =>
    invoke<HideMyEmailRefreshResult>('verify_hide_my_email_setup', { code, method }),
  refresh: () => invoke<HideMyEmailRefreshResult>('refresh_hide_my_email_aliases'),
  disconnect: () => invoke<void>('disconnect_hide_my_email'),
  removeAliases: () => invoke<void>('remove_hide_my_email_aliases'),
  copyAlias: (id: string) => invoke<void>('copy_hide_my_email_alias', { id })
};
