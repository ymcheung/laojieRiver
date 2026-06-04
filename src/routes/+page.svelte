<script lang="ts">
  import { ArrowRight, DatabaseZap, Shield, UserRoundCheck } from '@lucide/svelte';
  import Badge from '$lib/components/ui/badge/Badge.svelte';
</script>

<main class="min-h-screen px-6 py-6">
  <section class="mx-auto grid min-h-[calc(100vh-3rem)] max-w-6xl content-center gap-10">
    <div class="max-w-3xl">
      <Badge tone="accent">Local-first desktop vault</Badge>
      <h1 class="mt-5 text-5xl font-semibold tracking-normal text-[rgb(var(--foreground))]">
        LaoJie River
      </h1>
      <p class="mt-2 text-xl font-medium text-[rgb(var(--muted))]">老街溪</p>
      <p class="mt-6 max-w-2xl text-base leading-7 text-[rgb(var(--muted))]">
        A Tauri password manager and 2FA vault where Rust owns encryption, TOTP,
        clipboard control, and storage. The Svelte UI stays focused on interaction.
      </p>
      <div class="mt-8 flex flex-wrap gap-3">
        <a
          class="inline-flex h-10 cursor-pointer items-center justify-center gap-2 rounded-[var(--radius-md)] bg-[rgb(var(--primary))] px-4 text-sm font-medium text-[rgb(var(--primary-foreground))] transition-colors duration-200 hover:bg-[rgb(var(--primary-hover))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
          href="/onboarding"
        >
          Create vault
          <ArrowRight size={16} />
        </a>
        <a
          class="inline-flex h-10 cursor-pointer items-center justify-center rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] px-4 text-sm font-medium text-[rgb(var(--foreground))] transition-colors duration-200 hover:bg-[rgb(var(--surface-muted))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
          href="/unlock"
        >
          Unlock existing vault
        </a>
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      <article class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.86)] p-5">
        <Shield size={21} />
        <h2 class="mt-4 text-sm font-semibold">Vault identity is separate</h2>
        <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
          Neon Auth can identify the account, but the master password protects only the local vault.
        </p>
      </article>
      <article class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.86)] p-5">
        <DatabaseZap size={21} />
        <h2 class="mt-4 text-sm font-semibold">Encrypted blobs only</h2>
        <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
          SQLite and future sync should store encrypted payloads, nonces, and minimal metadata.
        </p>
      </article>
      <article class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.86)] p-5">
        <UserRoundCheck size={21} />
        <h2 class="mt-4 text-sm font-semibold">Accessible app shell</h2>
        <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
          Bits UI primitives and Tailwind tokens provide keyboard-first desktop workflows.
        </p>
      </article>
    </div>
  </section>
</main>
