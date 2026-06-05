<script lang="ts">
  import { goto } from '$app/navigation';
  import { createForm } from '@tanstack/svelte-form';
  import { AlertTriangle } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { vaultApi } from '$lib/features/vault/api';

  let errorMessage = $state('');

  const form = createForm(() => ({
    defaultValues: {
      masterPassword: '',
      confirmPassword: ''
    },
    validators: {
      onChange({ value }) {
        if (value.confirmPassword.length > 0 && value.masterPassword !== value.confirmPassword) {
          return 'Master passwords must match.';
        }

        return undefined;
      }
    },
    onSubmit: async ({ value }) => {
      errorMessage = '';

      try {
        await vaultApi.createVault(value.masterPassword);
        await goto('/vault');
      } catch (error) {
        errorMessage = error instanceof Error ? error.message : 'Could not create vault.';
      }
    }
  }));

  const formErrorMap = form.useStore((state) => state.errorMap);
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
        event.stopPropagation();
        void form.handleSubmit();
      }}
    >
      <form.Field
        name="masterPassword"
        validators={{
          onChange: ({ value }) =>
            value.length < 12 ? 'Use a longer master password or passphrase.' : undefined
        }}
      >
        {#snippet children(field)}
          <label class="grid gap-2 text-sm font-medium" for={field.name}>
            Master password
            <Input
              id={field.name}
              name={field.name}
              value={field.state.value}
              autocomplete="new-password"
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

      <form.Field
        name="confirmPassword"
        validators={{
          onChange: ({ value }) => (value.length === 0 ? 'Confirm your master password.' : undefined)
        }}
      >
        {#snippet children(field)}
          <label class="grid gap-2 text-sm font-medium" for={field.name}>
            Confirm master password
            <Input
              id={field.name}
              name={field.name}
              value={field.state.value}
              autocomplete="new-password"
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

      {#if formErrorMap.current.onChange}
        <p class="rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm text-[rgb(var(--foreground))]">
          {formErrorMap.current.onChange}
        </p>
      {/if}

      <div class="flex justify-end">
        <form.Subscribe selector={(state) => ({ canSubmit: state.canSubmit, isSubmitting: state.isSubmitting })}>
          {#snippet children(state)}
            <Button disabled={!state.canSubmit || state.isSubmitting} type="submit">
              {state.isSubmitting ? 'Creating...' : 'Create local vault'}
            </Button>
          {/snippet}
        </form.Subscribe>
      </div>
    </form>
  </section>
</main>
