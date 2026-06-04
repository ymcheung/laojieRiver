<script lang="ts">
  import Button from '$lib/components/ui/button/Button.svelte';
</script>

<main class="min-h-screen px-6 py-10">
  <section class="mx-auto max-w-4xl">
    <h1 class="text-3xl font-semibold">Settings</h1>
    <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
      Security, backup, import/export, clipboard, auto-lock, and future account settings.
    </p>

    <div class="mt-8 grid gap-4">
      {#each ['Security', 'Appearance', 'Backup', 'Import / Export', 'Clipboard', 'Auto-lock'] as section (section)}
        <article class="flex items-center justify-between gap-4 rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-5">
          <div>
            <h2 class="text-sm font-semibold">{section}</h2>
            <p class="mt-1 text-sm text-[rgb(var(--muted))]">Route and command wiring comes after vault core.</p>
          </div>
          <Button variant="secondary" size="sm">Open</Button>
        </article>
      {/each}
    </div>
  </section>
</main>

