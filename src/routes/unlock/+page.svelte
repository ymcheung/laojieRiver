<script lang="ts">
  import { goto } from '$app/navigation';
  import { createForm } from '@tanstack/svelte-form';
  import { LockKeyhole } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { vaultApi } from '$lib/features/vault/api';

  let errorMessage = $state('');

  const form = createForm(() => ({
    defaultValues: {
      masterPassword: ''
    },
    onSubmit: async ({ value }) => {
      errorMessage = '';

      try {
        await vaultApi.unlockVault(value.masterPassword);
        await goto('/vault');
      } catch (error) {
        errorMessage = error instanceof Error ? error.message : 'Could not unlock vault.';
      }
    }
  }));
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
        event.stopPropagation();
        void form.handleSubmit();
      }}
    >
      <form.Field
        name="masterPassword"
        validators={{
          onChange: ({ value }) => (value.length === 0 ? 'Master password is required.' : undefined)
        }}
      >
        {#snippet children(field)}
          <label class="grid gap-2 text-sm font-medium" for={field.name}>
            Master password
            <Input
              id={field.name}
              name={field.name}
              value={field.state.value}
              autocomplete="current-password"
              type="password"
              onblur={field.handleBlur}
              oninput={(event) => field.handleChange(event.currentTarget.value)}
            />
          </label>
          {#if field.state.meta.errors.length > 0}
            <p class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm text-[rgb(var(--foreground))]">
              {field.state.meta.errors.join(', ')}
            </p>
          {/if}
        {/snippet}
      </form.Field>

      {#if errorMessage}
        <p class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm text-[rgb(var(--foreground))]">
          {errorMessage}
        </p>
      {/if}

      <form.Subscribe selector={(state) => ({ canSubmit: state.canSubmit, isSubmitting: state.isSubmitting })}>
        {#snippet children(state)}
          <Button class="w-full" disabled={!state.canSubmit || state.isSubmitting} type="submit">
            {state.isSubmitting ? 'Unlocking...' : 'Unlock'}
          </Button>
        {/snippet}
      </form.Subscribe>
    </form>
  </section>
</main>
