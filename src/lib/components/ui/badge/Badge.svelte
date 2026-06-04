<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils/cn';

  let {
    tone = 'neutral',
    class: className,
    children
  }: {
    tone?: 'neutral' | 'accent' | 'warning';
    class?: string;
    children?: Snippet;
  } = $props();

  const tones = {
    neutral:
      'border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] text-[rgb(var(--foreground))]',
    accent:
      'border-[rgb(var(--accent)/0.26)] bg-[rgb(var(--accent)/0.12)] text-[rgb(var(--accent-foreground))]',
    warning:
      'border-[rgb(var(--border))] bg-[rgb(var(--surface-muted))] text-[rgb(var(--foreground))]'
  };
</script>

<span
  class={cn(
    'inline-flex items-center rounded-[var(--radius-sm)] border px-2 py-1 text-xs font-medium',
    tones[tone],
    className
  )}
>
  {#if children}
    {@render children()}
  {/if}
</span>
