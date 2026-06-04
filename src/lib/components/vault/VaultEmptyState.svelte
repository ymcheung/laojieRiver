<script lang="ts">
  import { CreditCard, FileText, IdCard, KeyRound, ShieldQuestion, Star } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';

  type EmptyStateIcon = 'key' | 'star' | 'note' | 'card' | 'identity' | 'totp';

  let {
    eyebrow,
    title,
    description,
    primaryAction,
    secondaryAction,
    icon = 'key'
  }: {
    eyebrow?: string;
    title: string;
    description: string;
    primaryAction?: string;
    secondaryAction?: string;
    icon?: EmptyStateIcon;
  } = $props();
</script>

<div
  class="grid min-h-72 place-items-center rounded-[var(--radius-md)] border border-dashed border-[rgb(var(--border))] bg-[rgb(var(--surface)/0.74)] p-6 text-center"
>
  <div class="max-w-sm">
    <div
      class="mx-auto flex h-12 w-12 items-center justify-center rounded-[var(--radius-md)] border border-[rgb(var(--border))] bg-[rgb(var(--surface))] text-[rgb(var(--accent-foreground))]"
      aria-hidden="true"
    >
      {#if icon === 'star'}
        <Star size={22} />
      {:else if icon === 'note'}
        <FileText size={22} />
      {:else if icon === 'card'}
        <CreditCard size={22} />
      {:else if icon === 'identity'}
        <IdCard size={22} />
      {:else if icon === 'totp'}
        <ShieldQuestion size={22} />
      {:else}
        <KeyRound size={22} />
      {/if}
    </div>

    {#if eyebrow}
      <p class="mt-5 text-xs font-medium uppercase tracking-normal text-[rgb(var(--muted))]">{eyebrow}</p>
    {/if}

    <h2 class="mt-2 text-lg font-semibold tracking-normal text-[rgb(var(--foreground))]">{title}</h2>
    <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">{description}</p>

    {#if primaryAction || secondaryAction}
      <div class="mt-5 flex flex-wrap justify-center gap-2">
        {#if primaryAction}
          <Button size="sm">{primaryAction}</Button>
        {/if}
        {#if secondaryAction}
          <Button size="sm" variant="secondary">{secondaryAction}</Button>
        {/if}
      </div>
    {/if}
  </div>
</div>
