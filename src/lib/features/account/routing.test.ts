import assert from 'node:assert/strict';
import { test } from 'node:test';
import { decideRoute, type RouteState } from './routing.ts';

const base: RouteState = {
  path: '/vault',
  sessionStatus: 'signed-out',
  demoMode: false,
  vault: { hasVault: false, unlocked: false }
};

test('routes authentication, demo, and vault lifecycle from one decision seam', () => {
  const cases: Array<[string, Partial<RouteState>, string | null]> = [
    ['signed-out real route', {}, '/auth'],
    ['signed-out explicit demo', { demoMode: true }, null],
    ['signed-in without vault', { sessionStatus: 'authenticated' }, '/onboarding'],
    [
      'signed-in locked vault',
      { sessionStatus: 'authenticated', vault: { hasVault: true, unlocked: false } },
      '/unlock'
    ],
    [
      'signed-in unlocked vault',
      { sessionStatus: 'authenticated', vault: { hasVault: true, unlocked: true } },
      null
    ],
    [
      'restored session resumes real vault lifecycle',
      { path: '/', sessionStatus: 'authenticated' },
      '/onboarding'
    ],
    ['persisted demo resumes demo vault', { path: '/', demoMode: true }, '/vault'],
    [
      'authenticated settings works without a vault',
      { path: '/settings', sessionStatus: 'authenticated' },
      null
    ],
    [
      'authenticated settings works with a locked vault',
      { path: '/settings', sessionStatus: 'authenticated', vault: { hasVault: true, unlocked: false } },
      null
    ],
    ['direct vault access never enables demo', {}, '/auth']
  ];

  for (const [name, state, expected] of cases) {
    assert.equal(decideRoute({ ...base, ...state }), expected, name);
  }
});
