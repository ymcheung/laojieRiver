<script lang="ts">
  import { page } from '$app/state';
  import { KeyRound, Lock, Plus, Search } from '@lucide/svelte';
  import Badge from '$lib/components/ui/badge/Badge.svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import VaultEmptyState from '$lib/components/vault/VaultEmptyState.svelte';
  import type { VaultItemSummary } from '$lib/features/vault/types';
  import { cn } from '$lib/utils/cn';

  type VaultSection = {
    id: string;
    label: string;
    href: string;
    description: string;
  };

  type EmptyStateContent = {
    eyebrow: string;
    title: string;
    description: string;
    primaryAction: string;
    secondaryAction?: string;
    icon: 'key' | 'star' | 'note' | 'card' | 'identity' | 'totp';
  };

  const sections: VaultSection[] = [
    {
      id: 'all',
      label: 'All Items',
      href: '/vault',
      description: 'All unlocked item summaries'
    },
    {
      id: 'favorites',
      label: 'Favorites',
      href: '/vault?section=favorites',
      description: 'Pinned item summaries'
    },
    {
      id: 'logins',
      label: 'Logins',
      href: '/vault?section=logins',
      description: 'Password login items'
    },
    {
      id: 'secure-notes',
      label: 'Secure Notes',
      href: '/vault?section=secure-notes',
      description: 'Encrypted note items'
    },
    {
      id: 'cards',
      label: 'Cards',
      href: '/vault?section=cards',
      description: 'Payment card items'
    },
    {
      id: 'identities',
      label: 'Identities',
      href: '/vault?section=identities',
      description: 'Identity profile items'
    },
    {
      id: '2fa',
      label: '2FA',
      href: '/vault?section=2fa',
      description: 'Items with TOTP codes'
    }
  ];

  const items: VaultItemSummary[] = [
    {
      id: 'sample-1',
      kind: 'login',
      title: 'GitHub',
      username: 'security@example.com',
      favorite: true,
      updatedAt: 'Today'
    },
    {
      id: 'sample-2',
      kind: 'login',
      title: 'Neon',
      username: 'sync@example.com',
      favorite: false,
      updatedAt: 'Yesterday'
    },
    {
      id: 'sample-3',
      kind: 'secure_note',
      title: 'Recovery codes',
      favorite: false,
      updatedAt: 'Last week'
    },
    {
      id: 'sample-4',
      kind: 'card',
      title: 'Travel card',
      favorite: false,
      updatedAt: 'May 28'
    },
    {
      id: 'sample-5',
      kind: 'identity',
      title: 'Personal identity',
      favorite: false,
      updatedAt: 'May 18'
    }
  ];

  const sectionId = $derived(page.url.searchParams.get('section') ?? 'all');
  const activeSection = $derived(sections.find((section) => section.id === sectionId) ?? sections[0]);
  const filteredItems = $derived(
    items.filter((item) => {
      if (activeSection.id === 'all') return true;
      if (activeSection.id === 'favorites') return item.favorite;
      if (activeSection.id === 'logins') return item.kind === 'login';
      if (activeSection.id === 'secure-notes') return item.kind === 'secure_note';
      if (activeSection.id === 'cards') return item.kind === 'card';
      if (activeSection.id === 'identities') return item.kind === 'identity';
      if (activeSection.id === '2fa') return false;
      return false;
    })
  );

  const emptyState = $derived(getEmptyState(activeSection.id));
  let frameElement: HTMLDivElement | undefined = $state();
  let leftPaneWidth = $state(440);

  const dividerWidth = 12;
  const minLeftPaneWidth = 320;
  const minRightPaneWidth = 512;
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

  function getEmptyState(section: string): EmptyStateContent {
    const copy: Record<string, EmptyStateContent> = {
      all: {
        eyebrow: 'Vault is ready',
        title: 'No items in this vault yet',
        description:
          'Create a login, secure note, card, identity, or 2FA entry after the Rust vault storage layer is connected.',
        primaryAction: 'New item',
        secondaryAction: 'Import backup',
        icon: 'key'
      },
      favorites: {
        eyebrow: 'Pinned items',
        title: 'No favorites yet',
        description:
          'Mark high-priority items as favorites so they stay easy to reach without exposing secret details in the list.',
        primaryAction: 'Browse all items',
        icon: 'star'
      },
      logins: {
        eyebrow: 'Passwords',
        title: 'No login items yet',
        description:
          'Add your first login once encrypted item creation is available. Password generation should stay on the Rust side.',
        primaryAction: 'Add login',
        icon: 'key'
      },
      'secure-notes': {
        eyebrow: 'Encrypted notes',
        title: 'No secure notes yet',
        description:
          'Use secure notes for recovery codes or private text that should be encrypted with the vault key.',
        primaryAction: 'Add secure note',
        icon: 'note'
      },
      cards: {
        eyebrow: 'Payment cards',
        title: 'No cards saved yet',
        description:
          'Store card details only after encrypted storage, field masking, and explicit reveal controls are implemented.',
        primaryAction: 'Add card',
        icon: 'card'
      },
      identities: {
        eyebrow: 'Profiles',
        title: 'No identities yet',
        description:
          'Identity items can hold profile details for form filling later. Keep summaries minimal while the vault is locked.',
        primaryAction: 'Add identity',
        icon: 'identity'
      },
      '2fa': {
        eyebrow: 'TOTP',
        title: 'No 2FA codes yet',
        description:
          'Paste an otpauth URL or add a secret manually once TOTP parsing is implemented in Rust.',
        primaryAction: 'Add 2FA code',
        secondaryAction: 'Paste otpauth URL',
        icon: 'totp'
      }
    };

    return copy[section] ?? copy.all;
  }

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

<section
  class="vault-shell relative min-h-screen bg-[rgb(var(--background))] px-4 pb-0 pt-16 sm:px-8 lg:px-0"
>
  <div class="fixed left-4 right-4 top-4 z-10 flex h-9 items-center justify-end gap-2.5 sm:left-auto sm:right-6">
    <div class="relative min-w-0 flex-1 sm:w-72 sm:flex-none lg:w-80">
      <Search
        class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-[rgb(var(--muted))]"
        size={16}
      />
      <Input inputSize="sm" aria-label="Search vault items" class="pl-9" placeholder="Search items" />
    </div>
    <Button class="h-9 px-3" variant="ghost" size="sm">
      <Lock size={16} />
      <span class="hidden sm:inline">Lock</span>
    </Button>
    <a
      class="inline-flex h-9 items-center rounded-[var(--radius-sm)] px-2.5 text-sm font-medium text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
      href="/settings"
    >
      Settings
    </a>
  </div>

  <nav
    class="mx-auto flex max-w-[78rem] flex-wrap gap-1 px-4 pb-4 text-sm sm:px-8 lg:absolute lg:top-16 lg:mx-0 lg:grid lg:w-[var(--vault-nav-width)] lg:px-0 lg:pb-0"
    aria-label="Vault sections"
  >
    {#each sections as section (section.id)}
      <a
        aria-current={activeSection.id === section.id ? 'page' : undefined}
        class={cn(
          'rounded-[var(--radius-md)] px-3 py-2 text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]',
          activeSection.id === section.id &&
            'bg-[rgb(var(--primary)/0.12)] font-medium text-[rgb(var(--accent-foreground))]'
        )}
        href={section.href}
      >
        {section.label}
      </a>
    {/each}
  </nav>

  <div
    bind:this={frameElement}
    class="mx-auto grid min-h-[calc(100vh-4rem)] max-w-[78rem] grid-cols-1 overflow-hidden rounded-tl-[2rem] lg:ml-[var(--vault-workspace-left)] lg:mr-0 lg:max-w-none lg:grid-cols-[var(--split-pane-columns)]"
    style:--split-pane-columns={splitPaneColumns}
  >
    <aside class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))]">
      <header class="border-b border-[rgb(var(--border))] px-5 py-5">
        <div class="flex flex-wrap items-center justify-end gap-3">
          <Button variant="secondary">
            <Plus size={16} />
            New item
          </Button>
        </div>
      </header>

      <section>
        <div class="border-b border-[rgb(var(--border))] px-5 py-4">
          <h1 class="text-base font-semibold">{activeSection.label}</h1>
          <p class="mt-1 text-sm text-[rgb(var(--muted))]">{activeSection.description}</p>
        </div>

        <div class="grid gap-2 p-3">
          {#each filteredItems as item (item.id)}
            <button
              class="grid cursor-pointer gap-1 rounded-[var(--radius-md)] border border-transparent p-3 text-left transition-colors duration-200 hover:border-[rgb(var(--border))] hover:bg-[rgb(var(--surface-muted))]"
              type="button"
            >
              <span class="flex items-center justify-between gap-3">
                <span class="font-medium">{item.title}</span>
                {#if item.favorite}
                  <Badge tone="accent">Favorite</Badge>
                {/if}
              </span>
              <span class="text-sm text-[rgb(var(--muted))]">{item.username}</span>
              <span class="text-xs text-[rgb(var(--muted))]">Updated {item.updatedAt}</span>
            </button>
          {:else}
            <VaultEmptyState
              eyebrow={emptyState.eyebrow}
              title={emptyState.title}
              description={emptyState.description}
              primaryAction={emptyState.primaryAction}
              secondaryAction={emptyState.secondaryAction}
              icon={emptyState.icon}
            />
          {/each}
        </div>
      </section>
    </aside>

    <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions - focusable separators are the ARIA pattern for resizable split panes. -->
    <div
      class="group relative hidden cursor-col-resize touch-none bg-[rgb(var(--surface-strong))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[rgb(var(--ring))] lg:block"
      role="separator"
      tabindex="0"
      aria-label="Resize vault columns"
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

    <main class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))] p-5 md:p-8">
      <div class="mx-auto max-w-3xl">
        {#if filteredItems.length === 0}
          <VaultEmptyState
            eyebrow="Nothing selected"
            title="Choose or create an item"
            description="The detail pane stays empty until an item summary is selected. Decrypted fields should be requested only for that selected item."
            primaryAction={emptyState.primaryAction}
            icon={emptyState.icon}
          />
        {:else}
          <Badge tone="warning">Scaffold mode</Badge>
          <div class="mt-4 flex items-start justify-between gap-6">
            <div>
              <h2 class="text-2xl font-semibold tracking-normal">Selected item detail</h2>
              <p class="mt-2 max-w-xl text-sm leading-6 text-[rgb(var(--muted))]">
                The UI shell is ready for narrow Tauri commands. Real decrypted fields should
                only be requested for the selected item after the Rust vault is unlocked.
              </p>
            </div>
            <div
              class="flex h-12 w-12 items-center justify-center rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))]"
              aria-hidden="true"
            >
              <KeyRound size={22} />
            </div>
          </div>

          <div class="mt-8 grid gap-4">
            {#each ['Username', 'Password', 'TOTP', 'Notes'] as label (label)}
              <div
                class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-4"
              >
                <p class="text-xs font-medium uppercase text-[rgb(var(--muted))]">{label}</p>
                <p class="mt-2 text-sm text-[rgb(var(--foreground))]">Connected to Rust command API later</p>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </main>
  </div>
</section>

<style>
  .vault-shell {
    --vault-edge: 1rem;
    --vault-nav-width: 8rem;
    --vault-nav-gap: 1rem;
    --vault-wireframe-left: max(var(--vault-edge), calc(50vw - 624px));
    --vault-nav-left: max(var(--vault-edge), calc(50vw - 800px));
    --vault-workspace-left: var(--vault-wireframe-left);
  }

  @media (min-width: 640px) {
    .vault-shell {
      --vault-edge: 2rem;
    }
  }

  @media (min-width: 1024px) {
    .vault-shell {
      padding-left: 0;
      padding-right: 0;
      --vault-workspace-left: max(
        var(--vault-wireframe-left),
        calc(var(--vault-nav-left) + var(--vault-nav-width) + var(--vault-nav-gap))
      );
    }

    .vault-shell nav {
      left: var(--vault-nav-left);
    }
  }
</style>
