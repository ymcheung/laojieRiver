import { env } from '$env/dynamic/public';
import { createAuthClient } from '@neondatabase/auth';
import { BetterAuthVanillaAdapter } from '@neondatabase/auth/vanilla';
import { disableDemoMode, enableDemoMode, isDemoModeEnabled } from '$lib/features/demoMode';
import { vaultApi } from '$lib/features/vault/api';
import type { VaultState } from '$lib/features/vault/types';
import { messages, type Locale } from './i18n';
import { realVaultDestination, type SessionStatus } from './routing';

type AccountUser = { id: string; email: string };
type AuthErrorShape = { code?: string; message?: string; status?: number; statusCode?: number };
type AuthResult<T> = { data?: T | null; error?: AuthErrorShape | null };

const pendingEmailKey = 'laojie-river:pending-email';
const pendingLogoutKey = 'laojie-river:pending-logout';
const localeKey = 'laojie-river:locale';
const lastAccountKey = 'laojie-river:last-account';
const emptyVault: VaultState = { hasVault: false, unlocked: false };

function createConfiguredClient(origin: string) {
  return createAuthClient(origin, {
    adapter: BetterAuthVanillaAdapter({ fetchOptions: { credentials: 'include' } })
  });
}

let client: ReturnType<typeof createConfiguredClient> | undefined;

function authClient() {
  if (client) return client;

  const baseUrl = env.PUBLIC_NEON_AUTH_BASE_URL?.trim() ?? '';
  if (!baseUrl) throw new Error('Authentication is not configured for this build.');

  const url = new URL(baseUrl);
  if (url.protocol !== 'https:') throw new Error('Authentication requires an HTTPS Neon Auth URL.');

  client = createConfiguredClient(url.toString().replace(/\/$/, ''));
  return client;
}

type ErrorKey =
  | 'errorNotConfigured'
  | 'errorRateLimit'
  | 'errorExpired'
  | 'errorInvalidCode'
  | 'errorNetwork'
  | 'errorGeneric';

function normalizeError(error: unknown): ErrorKey {
  const value = error as AuthErrorShape | undefined;
  const message = value?.message?.toLowerCase() ?? '';
  const code = value?.code?.toUpperCase() ?? '';
  const status = value?.status ?? value?.statusCode;

  if (
    code.includes('TOO_MANY') ||
    status === 429 ||
    message.includes('rate limit') ||
    message.includes('too many')
  ) {
    return 'errorRateLimit';
  }
  if (message.includes('not configured') || message.includes('https neon auth')) {
    return 'errorNotConfigured';
  }
  if (code.includes('EXPIRED') || message.includes('expired')) return 'errorExpired';
  if (
    code.includes('OTP') ||
    code.includes('CODE') ||
    message.includes('otp') ||
    message.includes('code') ||
    status === 400 ||
    status === 401
  ) {
    return 'errorInvalidCode';
  }
  if (error instanceof TypeError || message.includes('fetch') || message.includes('network')) {
    return 'errorNetwork';
  }
  return 'errorGeneric';
}

async function unwrap<T>(promise: Promise<AuthResult<T>>) {
  const result = await promise;
  if (result.error) throw result.error;
  return result.data ?? null;
}

class AccountSession {
  status = $state<SessionStatus>('loading');
  user = $state<AccountUser | null>(null);
  errorKey = $state<ErrorKey | ''>('');
  pendingLogout = $state(false);
  demoMode = $state(false);
  locale = $state<Locale>('en');
  vault = $state.raw<VaultState>(emptyVault);
  pendingEmail = $state('');

  get realVaultDestination() {
    return realVaultDestination(this.vault);
  }

  get error() {
    return this.errorKey ? messages[this.locale][this.errorKey] : '';
  }

  get notice() {
    return this.pendingLogout ? messages[this.locale].pendingLogout : '';
  }

  async initialize() {
    this.status = 'loading';
    this.errorKey = '';
    this.demoMode = isDemoModeEnabled();
    this.locale = localStorage.getItem(localeKey) === 'tw' ? 'tw' : 'en';
    this.pendingEmail = sessionStorage.getItem(pendingEmailKey) ?? '';

    try {
      await this.lockVault();
      if (localStorage.getItem(pendingLogoutKey)) {
        await unwrap(authClient().signOut() as Promise<AuthResult<unknown>>);
        localStorage.removeItem(pendingLogoutKey);
        this.pendingLogout = false;
      }

      const session = await unwrap(
        authClient().getSession() as Promise<AuthResult<{ user?: AccountUser }>>
      );
      const restoredUser = session?.user ?? null;
      const lastAccountId = sessionStorage.getItem(lastAccountKey);
      if (restoredUser && lastAccountId && restoredUser.id !== lastAccountId) {
        await this.discardVault();
      }
      this.user = restoredUser;
      this.status = this.user ? 'authenticated' : 'signed-out';
      if (this.user) sessionStorage.setItem(lastAccountKey, this.user.id);
    } catch (error) {
      this.user = null;
      if (localStorage.getItem(pendingLogoutKey)) {
        this.status = 'signed-out';
        this.pendingLogout = true;
      } else {
        this.status = 'error';
        this.errorKey = normalizeError(error);
      }
    }
  }

  async refreshVaultState() {
    this.vault = await vaultApi.getVaultState();
  }

  async revalidate() {
    if (this.status !== 'authenticated') return;

    try {
      const session = await unwrap(
        authClient().getSession() as Promise<AuthResult<{ user?: AccountUser }>>
      );
      if (!session?.user) {
        await this.lockVault();
        this.user = null;
        this.status = 'signed-out';
      } else if (this.user && session.user.id !== this.user.id) {
        await this.discardVault();
        this.user = session.user;
        sessionStorage.setItem(lastAccountKey, session.user.id);
      }
    } catch {
      // A temporary outage must not lock an already restored local vault.
    }
  }

  async requestOtp(email: string) {
    this.errorKey = '';
    const normalizedEmail = email.trim();

    try {
      await unwrap(
        authClient().emailOtp.sendVerificationOtp({
          email: normalizedEmail,
          type: 'sign-in'
        }) as Promise<AuthResult<unknown>>
      );
      this.pendingEmail = normalizedEmail;
      sessionStorage.setItem(pendingEmailKey, normalizedEmail);
    } catch (error) {
      this.errorKey = normalizeError(error);
      throw error;
    }
  }

  async verifyOtp(otp: string) {
    this.errorKey = '';

    try {
      const data = await unwrap(
        authClient().signIn.emailOtp({ email: this.pendingEmail, otp }) as Promise<
          AuthResult<{ user: AccountUser }>
        >
      );
      if (!data?.user) throw new Error('Authentication did not return a session.');
      const lastAccountId = sessionStorage.getItem(lastAccountKey);
      if (lastAccountId && lastAccountId !== data.user.id) await this.discardVault();

      this.user = data.user;
      sessionStorage.setItem(lastAccountKey, data.user.id);
      this.status = 'authenticated';
      this.cancelOtp();
      await this.refreshVaultState();
    } catch (error) {
      this.errorKey = normalizeError(error);
      throw error;
    }
  }

  cancelOtp() {
    this.pendingEmail = '';
    sessionStorage.removeItem(pendingEmailKey);
  }

  async enterDemo() {
    await this.lockVault();
    enableDemoMode();
    this.demoMode = true;
  }

  exitDemo() {
    disableDemoMode();
    this.demoMode = false;
  }

  setLocale(locale: Locale) {
    this.locale = locale;
    localStorage.setItem(localeKey, locale);
  }

  async lockVault() {
    await this.refreshVaultState();
    if (this.vault.hasVault && this.vault.unlocked) await vaultApi.lockVault();
    await this.refreshVaultState();
  }

  async discardVault() {
    await this.lockVault();
    await vaultApi.discardVault();
    await this.refreshVaultState();
  }

  async signOut() {
    this.errorKey = '';
    await this.lockVault();
    localStorage.setItem(pendingLogoutKey, 'pending');
    this.user = null;
    this.status = 'signed-out';

    try {
      await unwrap(authClient().signOut() as Promise<AuthResult<unknown>>);
      localStorage.removeItem(pendingLogoutKey);
      this.pendingLogout = false;
    } catch {
      this.pendingLogout = true;
    }
  }
}

export const accountSession = new AccountSession();
