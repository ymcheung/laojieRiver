import type { VaultState } from '$lib/features/vault/types';

export type SessionStatus = 'loading' | 'signed-out' | 'authenticated' | 'error';

export type RouteState = {
  path: string;
  sessionStatus: SessionStatus;
  demoMode: boolean;
  vault: VaultState;
};

const realVaultRoutes = new Set(['/onboarding', '/unlock', '/vault', '/settings']);

function realVaultDestination(vault: VaultState) {
  if (!vault.hasVault) return '/onboarding';
  return vault.unlocked ? '/vault' : '/unlock';
}

export function decideRoute({ path, sessionStatus, demoMode, vault }: RouteState): string | null {
  if (sessionStatus === 'loading') return null;

  if (demoMode) {
    if (path === '/vault' || path === '/settings') return null;
    return '/vault';
  }

  if (sessionStatus !== 'authenticated') {
    return realVaultRoutes.has(path) ? '/auth' : null;
  }

  if (path === '/settings') return null;

  const destination = realVaultDestination(vault);
  if (path === '/' || path === '/auth') return destination;
  if (path === '/onboarding' && vault.hasVault) return destination;
  if (path === '/unlock' && (!vault.hasVault || vault.unlocked)) return destination;
  if (path === '/vault' && destination !== '/vault') return destination;
  return null;
}

export { realVaultDestination };
