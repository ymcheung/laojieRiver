<script lang="ts">
  import { KeyRound, Lock, Plus, Search, ShieldCheck } from '@lucide/svelte';
  import Badge from '$lib/components/ui/badge/Badge.svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import type { VaultItemSummary } from '$lib/features/vault/types';

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
    }
  ];
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
      {#each ['All Items', 'Favorites', 'Logins', 'Secure Notes', 'Cards', 'Identities', '2FA'] as item (item)}
        <a
          class="rounded-[var(--radius-md)] px-3 py-2 text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))]"
          href="/vault"
        >
          {item}
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
          <h1 class="text-base font-semibold">All Items</h1>
          <p class="mt-1 text-sm text-[rgb(var(--muted))]">Summaries only while unlocked</p>
        </div>

        <div class="grid gap-2 p-3">
          {#each items as item (item.id)}
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
