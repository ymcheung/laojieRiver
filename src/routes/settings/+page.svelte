<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import {
    Archive,
    Brush,
    Clock,
    Download,
    FlaskConical,
    KeyRound,
    LockKeyhole,
    Monitor,
    ShieldCheck,
    UserRound
  } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import { messages, type Locale } from '$lib/features/account/i18n';
  import { accountSession } from '$lib/features/account/session.svelte';
  import SplitPaneSeparator from '$lib/features/splitPane/SplitPaneSeparator.svelte';
  import { createSplitPane } from '$lib/features/splitPane/createSplitPane.svelte';
  import { settingsSections, type SettingsSection } from '$lib/features/settings/sections';
  import { cn } from '$lib/utils/cn';

  const activeId = $derived(
    page.url.searchParams.get('section') ?? (accountSession.demoMode ? 'demo' : 'account')
  );
  const activeSection = $derived(
    settingsSections.find((section) => section.id === activeId) ?? settingsSections[0]
  );
  const splitPane = createSplitPane({
    minLeftPaneWidth: 320,
    minRightPaneWidth: 560
  });
  let signingOut = $state(false);
  const copy = $derived(messages[accountSession.locale]);

  async function turnOffDemoMode() {
    accountSession.exitDemo();
    await goto(
      accountSession.status === 'authenticated' ? accountSession.realVaultDestination : '/auth'
    );
  }

  async function turnOnDemoMode() {
    await accountSession.enterDemo();
    await goto('/vault');
  }

  async function signOut() {
    signingOut = true;
    await accountSession.signOut();
    signingOut = false;
    await goto('/auth');
  }

  function sectionLabel(section: SettingsSection) {
    return section.id === 'account' ? copy.account : section.label;
  }

  function sectionDescription(section: SettingsSection) {
    return section.id === 'account' ? copy.accountDescription : section.description;
  }
</script>

{#snippet settingsIcon(icon: SettingsSection['icon'])}
  {#if icon === 'account'}
    <UserRound size={18} />
  {:else if icon === 'security'}
    <ShieldCheck size={18} />
  {:else if icon === 'appearance'}
    <Brush size={18} />
  {:else if icon === 'backup'}
    <Archive size={18} />
  {:else if icon === 'import-export'}
    <Download size={18} />
  {:else if icon === 'clipboard'}
    <KeyRound size={18} />
  {:else if icon === 'demo'}
    <FlaskConical size={18} />
  {:else}
    <Clock size={18} />
  {/if}
{/snippet}

<main class="min-h-screen bg-[rgb(var(--background))] px-4 pb-0 pt-16 sm:px-8 lg:px-0">
  <div class="fixed left-4 right-4 top-4 z-10 flex h-9 items-center justify-end gap-2.5 sm:left-auto sm:right-6">
    <span
      class="inline-flex h-9 items-center rounded-[var(--radius-sm)] px-2.5 text-sm font-medium text-[rgb(var(--foreground))]"
    >
      Settings
    </span>
  </div>

  <section
    bind:this={splitPane.frameElement}
    class="mx-auto grid min-h-[calc(100vh-4rem)] max-w-[78rem] grid-cols-1 overflow-hidden rounded-tl-[2rem] lg:ml-[var(--frame-left)] lg:mr-0 lg:max-w-none lg:grid-cols-[var(--split-pane-columns)]"
    style:--frame-left="max(1rem, calc(50vw - 624px))"
    style:--split-pane-columns={splitPane.gridColumns}
  >
    <aside class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))]">
      <div class="border-b border-[rgb(var(--border))] px-5 py-5">
        <a
          class="inline-flex text-sm font-medium text-[rgb(var(--muted))] transition-colors duration-200 hover:text-[rgb(var(--foreground))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
          href="/vault"
        >
          Back to vault
        </a>
        <p class="mt-4 text-base font-semibold text-[rgb(var(--foreground))]">Settings</p>
        <p class="mt-1 text-sm leading-6 text-[rgb(var(--muted))]">
          Configure the vault shell without exposing secret data.
        </p>
      </div>

      <nav class="grid gap-1 p-3 text-sm" aria-label="Settings sections">
        {#each settingsSections as section (section.id)}
          <a
            aria-current={activeSection.id === section.id ? 'page' : undefined}
            class={cn(
              'grid grid-cols-[auto_minmax(0,1fr)] gap-3 rounded-[var(--radius-md)] px-3 py-3 text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))]',
              activeSection.id === section.id &&
                'bg-[rgb(var(--primary)/0.12)] font-medium text-[rgb(var(--accent-foreground))]'
            )}
            href={`/settings?section=${section.id}`}
          >
            <span
              class="mt-0.5 flex h-7 w-7 items-center justify-center rounded-[var(--radius-sm)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))]"
              aria-hidden="true"
            >
              {@render settingsIcon(section.icon)}
            </span>
            <span class="min-w-0">
              <span class="block">{sectionLabel(section)}</span>
              <span class="mt-1 block text-xs font-normal leading-5 text-[rgb(var(--muted))]">
                {sectionDescription(section)}
              </span>
            </span>
          </a>
        {/each}
      </nav>
    </aside>

    <SplitPaneSeparator
      label="Resize settings columns"
      min={splitPane.minLeftPaneWidth}
      max={splitPane.maxLeftPaneWidth}
      value={splitPane.constrainedLeftPaneWidth}
      onpointerdown={splitPane.startResize}
      onkeydown={splitPane.resizeWithKeyboard}
    />

    <section class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))] p-5 md:p-8">
      <div class="mx-auto max-w-3xl">
        <div class="flex items-start justify-between gap-6">
          <div>
            <p class="text-xs font-medium uppercase tracking-normal text-[rgb(var(--muted))]">Settings</p>
            <h1 class="mt-2 text-2xl font-semibold tracking-normal text-[rgb(var(--foreground))]">
              {sectionLabel(activeSection)}
            </h1>
            <p class="mt-2 max-w-xl text-sm leading-6 text-[rgb(var(--muted))]">
              {sectionDescription(activeSection)}{#if activeSection.id !== 'account'} Command wiring comes after the vault core and platform integrations.{/if}
            </p>
          </div>
          <div
            class="flex h-12 w-12 items-center justify-center rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] text-[rgb(var(--accent-foreground))]"
            aria-hidden="true"
          >
            {@render settingsIcon(activeSection.icon)}
          </div>
        </div>

        <div class="mt-8 grid gap-4">
          {#if activeSection.id === 'account'}
            <section
              class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-4"
            >
              <div class="grid gap-5">
                {#if !accountSession.demoMode && accountSession.user}
                  <div>
                    <p class="text-sm text-[rgb(var(--muted))]">{copy.signedInAs}</p>
                    <p class="mt-1 text-sm font-medium">{accountSession.user.email}</p>
                  </div>
                {/if}

                <label class="grid gap-2 text-sm font-medium" for="account-locale">
                  {copy.language}
                  <select
                    id="account-locale"
                    class="h-10 rounded-[var(--radius-md)] border border-[rgb(var(--input))] bg-[rgb(var(--surface))] px-3 text-sm"
                    value={accountSession.locale}
                    onchange={(event) => accountSession.setLocale(event.currentTarget.value as Locale)}
                  >
                    <option value="en">{copy.english}</option>
                    <option value="tw">{copy.chinese}</option>
                  </select>
                </label>

                {#if !accountSession.demoMode && accountSession.user}
                  <div>
                    <Button
                      variant="secondary"
                      disabled={signingOut}
                      onclick={() => void signOut()}
                    >
                      {signingOut ? copy.signingOut : copy.signOut}
                    </Button>
                  </div>
                {/if}
              </div>
            </section>
          {:else if activeSection.id === 'demo'}
            <section
              class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-4"
            >
              <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                <div>
                  <h2 class="text-sm font-semibold">Demo vault access</h2>
                  <p class="mt-1 text-sm leading-6 text-[rgb(var(--muted))]">
                    Demo mode uses local sample data only. Turn it off to return to the startup
                    screen for real user onboarding and future Neon Auth sign-in.
                  </p>
                </div>
                <div class="flex shrink-0 items-center gap-3">
                  <span
                    class={cn(
                      'inline-flex h-8 items-center rounded-[var(--radius-sm)] border px-3 text-sm font-medium',
                      accountSession.demoMode
                        ? 'border-[rgb(var(--primary)/0.28)] bg-[rgb(var(--primary)/0.12)] text-[rgb(var(--accent-foreground))]'
                        : 'border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] text-[rgb(var(--muted))]'
                    )}
                  >
                    {accountSession.demoMode ? 'On' : 'Off'}
                  </span>
                  <Button
                    variant="secondary"
                    size="sm"
                    onclick={() =>
                      void (accountSession.demoMode ? turnOffDemoMode() : turnOnDemoMode())}
                  >
                    {accountSession.demoMode ? 'Turn off' : 'Turn on'}
                  </Button>
                </div>
              </div>
            </section>
          {:else}
            <section
              class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-4"
            >
              <div class="flex items-center justify-between gap-4">
                <div>
                  <h2 class="text-sm font-semibold">Status</h2>
                  <p class="mt-1 text-sm leading-6 text-[rgb(var(--muted))]">
                    This settings surface is ready for Tauri command integration.
                  </p>
                </div>
                <Button variant="secondary" size="sm">Configure</Button>
              </div>
            </section>
          {/if}

          <section
            class="rounded-[var(--radius-md)] border border-dashed border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.74)] p-4"
          >
            <div class="flex gap-3">
              <div
                class="flex h-9 w-9 shrink-0 items-center justify-center rounded-[var(--radius-sm)] border border-[rgb(var(--border))] text-[rgb(var(--muted))]"
                aria-hidden="true"
              >
                <Monitor size={17} />
              </div>
              <div>
                <h2 class="text-sm font-semibold">Wireframe mode</h2>
                <p class="mt-1 text-sm leading-6 text-[rgb(var(--muted))]">
                  The split pane matches the provided light and dark wireframes while preserving accessible
                  navigation, focus rings, and stable hover states.
                </p>
              </div>
            </div>
          </section>

          <section
            class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-4"
          >
            <div class="flex gap-3">
              <div
                class="flex h-9 w-9 shrink-0 items-center justify-center rounded-[var(--radius-sm)] border border-[rgb(var(--border))] text-[rgb(var(--muted))]"
                aria-hidden="true"
              >
                <LockKeyhole size={17} />
              </div>
              <div>
                <h2 class="text-sm font-semibold">Security boundary</h2>
                <p class="mt-1 text-sm leading-6 text-[rgb(var(--muted))]">
                  Settings should change behavior through narrow Rust commands and must not store secret values
                  in browser storage.
                </p>
              </div>
            </div>
          </section>
        </div>
      </div>
    </section>
  </section>
</main>
