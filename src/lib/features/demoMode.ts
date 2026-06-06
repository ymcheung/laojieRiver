const demoModeKey = 'laojie-river:demo-mode';

export function isDemoModeEnabled() {
  if (typeof localStorage === 'undefined') return false;

  return localStorage.getItem(demoModeKey) === 'enabled';
}

export function enableDemoMode() {
  localStorage.setItem(demoModeKey, 'enabled');
}

export function disableDemoMode() {
  localStorage.removeItem(demoModeKey);
}
