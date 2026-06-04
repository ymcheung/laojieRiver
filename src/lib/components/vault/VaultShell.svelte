<script lang="ts">
  import { page } from '$app/state';
  import { KeyRound, Lock, Plus, Search, ShieldCheck } from '@lucide/svelte';
  import Badge from '$lib/components/ui/badge/Badge.svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import type { VaultItemSummary } from '$lib/features/vault/types';
  import { cn } from '$lib/utils/cn';

  type VaultSection = {
    id: string;
    label: string;
    href: string;
    description: string;
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
      if (activeSection.id === '2fa') return item.id === 'sample-1';
      return false;
    })
  );
</script>

<section class="grid min-h-screen bg-[rgb(var(--background))] lg:grid-cols-[16rem_minmax(0,1fr)]">
  <aside class="border-b border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.92)] px-4 py-5 lg:border-b-0 lg:border-r">
    <div class="flex items-center gap-3">
      <div
        class="flex h-10 w-10 items-center justify-center rounded-[var(--radius-md)] bg-[rgb(var(--primary))] text-[rgb(var(--primary-foreground))]"
        aria-hidden="true"
      >
        <ShieldCheck size={20} />
      </div>
      <div>
        <p class="text-sm font-semibold leading-5">LaoJie River</p>
        <p class="text-xs text-[rgb(var(--muted))]">老街溪</p>
      </div>
    </div>

    <nav class="mt-6 flex flex-wrap gap-1 text-sm lg:mt-8 lg:grid" aria-label="Vault sections">
      {#each sections as section (section.id)}
        <a
          aria-current={activeSection.id === section.id ? 'page' : undefined}
          class={cn(
            'rounded-[var(--radius-md)] px-3 py-2 text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))]',
            activeSection.id === section.id &&
              'bg-[rgb(var(--primary)/0.12)] font-medium text-[rgb(var(--accent-foreground))]'
          )}
          href={section.href}
        >
          {section.label}
        </a>
      {/each}
    </nav>
  </aside>

  <main class="grid min-w-0 grid-rows-[auto_minmax(0,1fr)]">
    <header
      class="flex flex-wrap items-center gap-3 border-b border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.78)] px-5 py-4 backdrop-blur"
    >
      <div class="relative min-w-56 flex-1">
        <Search
          class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-[rgb(var(--muted))]"
          size={17}
        />
        <Input aria-label="Search vault items" class="pl-9" placeholder="Search items or actions" />
      </div>
      <Button variant="secondary">
        <Plus size={16} />
        New item
      </Button>
      <Button variant="ghost">
        <Lock size={16} />
        Lock
      </Button>
    </header>

    <div class="grid min-h-0 xl:grid-cols-[22rem_minmax(0,1fr)]">
      <section class="overflow-auto border-r border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.62)]">
        <div class="border-b border-[rgb(var(--border))] px-5 py-4">
          <h1 class="text-base font-semibold">{activeSection.label}</h1>
          <p class="mt-1 text-sm text-[rgb(var(--muted))]">{activeSection.description}</p>
        </div>

        <div class="grid gap-2 p-3">
          {#each filteredItems as item (item.id)}
            <button
              class="grid cursor-pointer gap-1 rounded-[var(--radius-md)] border border-transparent p-3 text-left transition-colors duration-200 hover:border-[rgb(var(--border))] hover:bg-[rgb(var(--surface))]"
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
            <div class="rounded-[var(--radius-md)] border border-dashed border-[rgb(var(--border))] p-4">
              <p class="text-sm font-medium">No items in {activeSection.label.toLowerCase()}.</p>
              <p class="mt-1 text-sm text-[rgb(var(--muted))]">
                Add or import vault data after the Rust storage layer is connected.
              </p>
            </div>
          {/each}
        </div>
      </section>

      <section class="min-w-0 overflow-auto border-t border-[rgb(var(--border))] p-5 md:p-8 xl:border-t-0">
        <div class="mx-auto max-w-3xl">
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
        </div>
      </section>
    </div>
  </main>
</section>
