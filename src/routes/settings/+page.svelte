<script lang="ts">
  import { page } from '$app/state';
  import {
    Archive,
    Brush,
    Clock,
    Download,
    KeyRound,
    LockKeyhole,
    Monitor,
    ShieldCheck
  } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import { cn } from '$lib/utils/cn';

  type SettingsSection = {
    id: string;
    label: string;
    description: string;
    icon: 'security' | 'appearance' | 'backup' | 'import-export' | 'clipboard' | 'auto-lock';
  };

  const sections: SettingsSection[] = [
    {
      id: 'security',
      label: 'Security',
      description: 'Master password, vault lock behavior, and recovery limits.',
      icon: 'security'
    },
    {
      id: 'appearance',
      label: 'Appearance',
      description: 'Theme, density, and window preferences.',
      icon: 'appearance'
    },
    {
      id: 'backup',
      label: 'Backup',
      description: 'Encrypted backup destinations and restore checks.',
      icon: 'backup'
    },
    {
      id: 'import-export',
      label: 'Import / Export',
      description: 'CSV, encrypted archive, and plaintext export warnings.',
      icon: 'import-export'
    },
    {
      id: 'clipboard',
      label: 'Clipboard',
      description: 'Copy timeout and clear-after-copy behavior.',
      icon: 'clipboard'
    },
    {
      id: 'auto-lock',
      label: 'Auto-lock',
      description: 'Inactivity, sleep, background, and restart locking.',
      icon: 'auto-lock'
    }
  ];

  const activeId = $derived(page.url.searchParams.get('section') ?? 'security');
  const activeSection = $derived(sections.find((section) => section.id === activeId) ?? sections[0]);
  let frameElement: HTMLElement | undefined = $state();
  let leftPaneWidth = $state(440);

  const dividerWidth = 12;
  const minLeftPaneWidth = 320;
  const minRightPaneWidth = 560;
  const maxLeftPaneWidth = $derived.by(() => {
    if (!frameElement) return 560;
    return Math.max(minLeftPaneWidth, frameElement.clientWidth - dividerWidth - minRightPaneWidth);
  });
  const constrainedLeftPaneWidth = $derived(
    Math.min(Math.max(leftPaneWidth, minLeftPaneWidth), maxLeftPaneWidth)
  );
  const splitPaneColumns = $derived(
    `${constrainedLeftPaneWidth}px ${dividerWidth}px minmax(${minRightPaneWidth}px, 1fr)`
  );

  function resizeLeftPane(clientX: number) {
    if (!frameElement) return;

    const frameRect = frameElement.getBoundingClientRect();
    leftPaneWidth = Math.min(
      Math.max(clientX - frameRect.left, minLeftPaneWidth),
      frameRect.width - dividerWidth - minRightPaneWidth
    );
  }

  function startResize(event: PointerEvent) {
    if (!frameElement) return;

    event.preventDefault();
    resizeLeftPane(event.clientX);

    function handlePointerMove(moveEvent: PointerEvent) {
      resizeLeftPane(moveEvent.clientX);
    }

    function stopResize() {
      window.removeEventListener('pointermove', handlePointerMove);
      window.removeEventListener('pointerup', stopResize);
    }

    window.addEventListener('pointermove', handlePointerMove);
    window.addEventListener('pointerup', stopResize, { once: true });
  }

  function resizeWithKeyboard(event: KeyboardEvent) {
    if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;

    event.preventDefault();
    const direction = event.key === 'ArrowLeft' ? -1 : 1;
    const step = event.shiftKey ? 48 : 24;
    leftPaneWidth = Math.min(Math.max(leftPaneWidth + direction * step, minLeftPaneWidth), maxLeftPaneWidth);
  }
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
  {:else}
    <Clock size={18} />
  {/if}
{/snippet}

<main class="min-h-screen bg-[rgb(var(--background))] px-4 pb-0 pt-16 sm:px-8 lg:px-0">
  <span
    class="fixed right-6 top-5 z-10 rounded-[var(--radius-sm)] px-2 py-1 text-sm font-medium text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
  >
    Settings
  </span>

  <section
    bind:this={frameElement}
    class="mx-auto grid min-h-[calc(100vh-4rem)] max-w-[78rem] grid-cols-1 overflow-hidden rounded-tl-[2rem] lg:ml-[var(--frame-left)] lg:mr-0 lg:max-w-none lg:grid-cols-[var(--split-pane-columns)]"
    style:--frame-left="max(1rem, calc(50vw - 624px))"
    style:--split-pane-columns={splitPaneColumns}
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
        {#each sections as section (section.id)}
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

    <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions - focusable separators are the ARIA pattern for resizable split panes. -->
    <div
      class="group relative hidden cursor-col-resize touch-none bg-[rgb(var(--surface-strong))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[rgb(var(--ring))] lg:block"
      role="separator"
      tabindex="0"
      aria-label="Resize settings columns"
      aria-orientation="vertical"
      aria-valuemin={minLeftPaneWidth}
      aria-valuemax={maxLeftPaneWidth}
      aria-valuenow={constrainedLeftPaneWidth}
      onpointerdown={startResize}
      onkeydown={resizeWithKeyboard}
    >
      <span
        class="absolute inset-y-0 left-1/2 w-0.5 -translate-x-1/2 bg-transparent transition-colors duration-200 group-hover:bg-[rgb(var(--muted))] group-focus-visible:bg-[rgb(var(--muted))]"
        aria-hidden="true"
      ></span>
    </div>

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
