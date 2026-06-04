<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils/cn';

  type Variant = 'default' | 'secondary' | 'ghost' | 'danger';
  type Size = 'sm' | 'md' | 'icon';

  let {
    variant = 'default',
    size = 'md',
    class: className,
    children,
    ...rest
  }: HTMLButtonAttributes & {
    variant?: Variant;
    size?: Size;
    children?: Snippet;
  } = $props();

  const base =
    'inline-flex cursor-pointer items-center justify-center gap-2 rounded-[var(--radius-md)] font-medium transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))] disabled:pointer-events-none disabled:opacity-50';

  const variants: Record<Variant, string> = {
    default:
      'bg-[rgb(var(--primary))] text-[rgb(var(--primary-foreground))] hover:bg-[rgb(var(--primary-hover))]',
    secondary:
      'border border-[rgb(var(--border))] bg-[rgb(var(--surface))] text-[rgb(var(--foreground))] hover:bg-[rgb(var(--surface-muted))]',
    ghost: 'text-[rgb(var(--foreground))] hover:bg-[rgb(var(--surface-muted))]',
    danger: 'bg-[rgb(var(--danger))] text-white hover:bg-[rgb(var(--danger)/0.88)]'
  };

  const sizes: Record<Size, string> = {
    sm: 'h-8 px-3 text-sm',
    md: 'h-10 px-4 text-sm',
    icon: 'h-9 w-9'
  };
</script>

<button
  class={cn(base, variants[variant], sizes[size], typeof className === 'string' ? className : undefined)}
  {...rest}
>
  {#if children}
    {@render children()}
  {/if}
</button>
