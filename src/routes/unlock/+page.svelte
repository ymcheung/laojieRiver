<script lang="ts">
  import { goto } from '$app/navigation';
  import { LockKeyhole } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { vaultApi } from '$lib/features/vault/api';

  let masterPassword = $state('');
  let errorMessage = $state('');
  let isUnlocking = $state(false);

  async function unlockVault() {
    if (masterPassword.length === 0 || isUnlocking) return;

    errorMessage = '';
    isUnlocking = true;

    try {
      await vaultApi.unlockVault(masterPassword);
      await goto('/vault');
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Could not unlock vault.';
    } finally {
      isUnlocking = false;
    }
  }
</script>

<main class="grid min-h-screen place-items-center px-6 py-10">
  <section class="w-full max-w-md rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-6 shadow-sm">
    <div
      class="flex h-11 w-11 items-center justify-center rounded-[var(--radius-md)] bg-[rgb(var(--surface-muted))]"
      aria-hidden="true"
    >
      <LockKeyhole size={20} />
    </div>
    <h1 class="mt-5 text-2xl font-semibold">Unlock vault</h1>
    <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
      Your service account can be signed in while this local vault remains locked.
    </p>

    <form
      class="mt-6 grid gap-4"
      onsubmit={(event) => {
        event.preventDefault();
        void unlockVault();
      }}
    >
      <label class="grid gap-2 text-sm font-medium">
        Master password
        <Input bind:value={masterPassword} autocomplete="current-password" type="password" />
      </label>

      {#if errorMessage}
        <p class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm text-[rgb(var(--foreground))]">
          {errorMessage}
        </p>
      {/if}

      <Button class="w-full" disabled={masterPassword.length === 0 || isUnlocking} type="submit">
        {isUnlocking ? 'Unlocking...' : 'Unlock'}
      </Button>
    </form>
  </section>
</main>
