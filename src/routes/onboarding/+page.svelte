<script lang="ts">
  import { AlertTriangle } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';

  let masterPassword = $state('');
  let confirmPassword = $state('');
  let passwordsMatch = $derived(masterPassword.length > 0 && masterPassword === confirmPassword);
</script>

<main class="grid min-h-screen place-items-center px-6 py-10">
  <section class="w-full max-w-lg rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] p-6 shadow-sm">
    <h1 class="text-2xl font-semibold">Create your vault</h1>
    <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">
      The master password unlocks encrypted secrets locally. It is separate from any future service account password.
    </p>

    <div class="mt-6 grid gap-4">
      <label class="grid gap-2 text-sm font-medium">
        Master password
        <Input bind:value={masterPassword} autocomplete="new-password" type="password" />
      </label>
      <label class="grid gap-2 text-sm font-medium">
        Confirm master password
        <Input bind:value={confirmPassword} autocomplete="new-password" type="password" />
      </label>
    </div>

    <div class="mt-5 flex gap-3 rounded-[var(--radius-md)] border border-[rgb(var(--warning)/0.32)] bg-[rgb(var(--warning)/0.12)] p-4">
      <AlertTriangle class="mt-0.5 shrink-0 text-[rgb(var(--accent-foreground))]" size={18} />
      <p class="text-sm leading-6 text-[rgb(var(--foreground))]">
        We cannot recover your vault master password. Add a recovery-key system before offering account recovery for vault data.
      </p>
    </div>

    <div class="mt-6 flex justify-end">
      <Button disabled={!passwordsMatch}>Create local vault</Button>
    </div>
  </section>
</main>
