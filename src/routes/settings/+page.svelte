<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import {
    Archive,
    Brush,
    Clock,
    Download,
    FlaskConical,
    KeyRound,
    LockKeyhole,
    Monitor,
    ShieldCheck
  } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import { disableDemoMode, isDemoModeEnabled } from '$lib/features/demoMode';
  import SplitPaneSeparator from '$lib/features/splitPane/SplitPaneSeparator.svelte';
  import { createSplitPane } from '$lib/features/splitPane/createSplitPane.svelte';
  import { settingsSections, type SettingsSection } from '$lib/features/settings/sections';
  import { cn } from '$lib/utils/cn';

  const activeId = $derived(page.url.searchParams.get('section') ?? 'security');
  const activeSection = $derived(
    settingsSections.find((section) => section.id === activeId) ?? settingsSections[0]
  );
  const splitPane = createSplitPane({
    minLeftPaneWidth: 320,
    minRightPaneWidth: 560
  });
  let demoModeEnabled = $state(false);

  async function turnOffDemoMode() {
    disableDemoMode();
    demoModeEnabled = false;
    await goto('/');
  }

  onMount(() => {
    demoModeEnabled = isDemoModeEnabled();
  });
</script>

{#snippet settingsIcon(icon: SettingsSection['icon'])}
  {#if icon === 'security'}
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
              <span class="block">{section.label}</span>
              <span class="mt-1 block text-xs font-normal leading-5 text-[rgb(var(--muted))]">
                {section.description}
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
              {activeSection.label}
            </h1>
            <p class="mt-2 max-w-xl text-sm leading-6 text-[rgb(var(--muted))]">
              {activeSection.description} Command wiring comes after the vault core and platform integrations.
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
          {#if activeSection.id === 'demo'}
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
                      demoModeEnabled
                        ? 'border-[rgb(var(--primary)/0.28)] bg-[rgb(var(--primary)/0.12)] text-[rgb(var(--accent-foreground))]'
                        : 'border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] text-[rgb(var(--muted))]'
                    )}
                  >
                    {demoModeEnabled ? 'On' : 'Off'}
                  </span>
                  <Button
                    disabled={!demoModeEnabled}
                    variant="secondary"
                    size="sm"
                    onclick={() => void turnOffDemoMode()}
                  >
                    Turn off
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
