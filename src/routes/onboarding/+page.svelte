<script lang="ts">
  import { goto } from '$app/navigation';
  import { AlertTriangle } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { vaultApi } from '$lib/features/vault/api';

  let masterPassword = $state('');
  let confirmPassword = $state('');
  let passwordsMatch = $derived(masterPassword.length > 0 && masterPassword === confirmPassword);
  let errorMessage = $state('');
  let isCreating = $state(false);

  async function createVault() {
    if (!passwordsMatch || isCreating) return;

    errorMessage = '';
    isCreating = true;

    try {
      await vaultApi.createVault(masterPassword);
      await goto('/vault');
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Could not create vault.';
    } finally {
      isCreating = false;
    }
  }
</script>

<main class="grid min-h-screen place-items-center px-6 py-10">
  <section class="w-full max-w-lg rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-6 shadow-sm">
    <h1 class="text-2xl font-semibold">Create your vault</h1>
    <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
      The master password unlocks encrypted secrets locally. It is separate from any future service account password.
    </p>

    <form
      class="mt-6 grid gap-4"
      onsubmit={(event) => {
        event.preventDefault();
        void createVault();
      }}
    >
      <label class="grid gap-2 text-sm font-medium">
        Master password
        <Input bind:value={masterPassword} autocomplete="new-password" type="password" />
      </label>
      <label class="grid gap-2 text-sm font-medium">
        Confirm master password
        <Input bind:value={confirmPassword} autocomplete="new-password" type="password" />
      </label>

      <div class="flex gap-3 rounded-[var(--radius-md)] border border-[rgb(var(--warning)/0.32)] bg-[rgb(var(--warning)/0.12)] p-4">
        <AlertTriangle class="mt-0.5 shrink-0 text-[rgb(var(--accent-foreground))]" size={18} />
        <p class="text-sm leading-6 text-[rgb(var(--foreground))]">
          We cannot recover your vault master password. Add a recovery-key system before offering account recovery for vault data.
        </p>
      </div>

      {#if errorMessage}
        <p class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm text-[rgb(var(--foreground))]">
          {errorMessage}
        </p>
      {/if}

      <div class="flex justify-end">
        <Button disabled={!passwordsMatch || isCreating} type="submit">
          {isCreating ? 'Creating...' : 'Create local vault'}
        </Button>
      </div>
    </form>
  </section>
</main>
