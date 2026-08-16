<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import '../app.css';
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { messages } from '$lib/features/account/i18n';
  import { decideRoute } from '$lib/features/account/routing';
  import { accountSession } from '$lib/features/account/session.svelte';

  let { children }: { children: Snippet } = $props();

  const copy = $derived(messages[accountSession.locale]);
  const redirect = $derived(
    decideRoute({
      path: page.url.pathname,
      sessionStatus: accountSession.status,
      demoMode: accountSession.demoMode,
      vault: accountSession.vault
    })
  );

  $effect(() => {
    if (redirect && redirect !== page.url.pathname) void goto(redirect, { replaceState: true });
  });

  onMount(() => {
    void accountSession.initialize();
  });

  function revalidateVisibleSession() {
    if (document.visibilityState === 'visible') void accountSession.revalidate();
  }
</script>

<svelte:document onvisibilitychange={revalidateVisibleSession} />

{#if accountSession.status === 'loading' || redirect}
  <main class="grid min-h-screen place-items-center px-6 text-center" aria-live="polite">
    <p class="text-sm text-[rgb(var(--muted))]">{copy.loading}</p>
  </main>
{:else}
  {@render children()}
{/if}
