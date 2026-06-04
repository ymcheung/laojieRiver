<script lang="ts">
  import { LockKeyhole } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';

  let masterPassword = $state('');
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

    <label class="mt-6 grid gap-2 text-sm font-medium">
      Master password
      <Input bind:value={masterPassword} autocomplete="current-password" type="password" />
    </label>

    <Button class="mt-6 w-full" disabled={masterPassword.length === 0}>Unlock</Button>
  </section>
</main>
