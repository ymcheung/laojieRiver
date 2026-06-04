<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils/cn';

  let {
    class: className,
    value = $bindable(),
    ...rest
  }: HTMLInputAttributes = $props();
</script>

<input
  bind:value
  class={cn(
    'h-10 w-full rounded-[var(--radius-md)] border border-[rgb(var(--input))] bg-[rgb(var(--surface))] px-3 text-sm text-[rgb(var(--foreground))] outline-none transition-colors duration-200 placeholder:text-[rgb(var(--muted))] focus:border-[rgb(var(--ring))] focus:ring-2 focus:ring-[rgb(var(--ring)/0.18)] disabled:cursor-not-allowed disabled:opacity-60',
    typeof className === 'string' ? className : undefined
  )}
  {...rest}
/>
