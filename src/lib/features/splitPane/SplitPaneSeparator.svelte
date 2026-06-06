<script lang="ts">
  let {
    label,
    min,
    max,
    value,
    onpointerdown,
    onkeydown
  }: {
    label: string;
    min: number;
    max: number;
    value: number;
    onpointerdown: (event: PointerEvent) => void;
    onkeydown: (event: KeyboardEvent) => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions - focusable separators are the ARIA pattern for resizable split panes. -->
<div
  class="group relative hidden cursor-col-resize touch-none bg-[rgb(var(--surface-strong))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[rgb(var(--ring))] lg:block"
  role="separator"
  tabindex="0"
  aria-label={label}
  aria-orientation="vertical"
  aria-valuemin={min}
  aria-valuemax={max}
  aria-valuenow={value}
  {onpointerdown}
  {onkeydown}
>
  <span
    class="absolute inset-y-0 left-1/2 w-0.5 -translate-x-1/2 bg-transparent transition-colors duration-200 group-hover:bg-[rgb(var(--muted))] group-focus-visible:bg-[rgb(var(--muted))]"
    aria-hidden="true"
  ></span>
</div>
