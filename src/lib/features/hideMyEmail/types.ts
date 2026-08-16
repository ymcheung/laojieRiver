export type HideMyEmailAliasSummary = {
  id: string;
  address: string;
  label?: string;
  origin?: string;
  isActive: boolean;
  updatedAt?: string;
};

export type HideMyEmailStatus = {
  connectionState: 'disconnected' | 'connected' | 'expired' | 'syncing';
  setupAvailable: boolean;
  maskedAppleId?: string;
  lastRefreshAt?: string;
};

export type HideMyEmailVerificationMethod = 'trustedDevice' | 'sms';

export type HideMyEmailSetupResult = {
  requiresVerification: boolean;
  methods: HideMyEmailVerificationMethod[];
};

export type HideMyEmailRefreshResult = {
  added: number;
  updated: number;
  unchanged: number;
  markedInactive: number;
};
