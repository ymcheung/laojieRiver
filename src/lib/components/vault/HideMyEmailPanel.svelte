<script lang="ts">
  import { goto } from '$app/navigation';
  import { Copy, Mail, RefreshCw, ShieldAlert, Unplug } from '@lucide/svelte';
  import Badge from '$lib/components/ui/badge/Badge.svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { accountSession } from '$lib/features/account/session.svelte';
  import { hideMyEmailApi } from '$lib/features/hideMyEmail/api';
  import { demoHideMyEmailAliases } from '$lib/features/hideMyEmail/demoAliases';
  import { filterHideMyEmailAliases } from '$lib/features/hideMyEmail/filterAliases';
  import { hideMyEmailMessages } from '$lib/features/hideMyEmail/i18n';
  import type {
    HideMyEmailAliasSummary,
    HideMyEmailRefreshResult,
    HideMyEmailStatus,
    HideMyEmailVerificationMethod
  } from '$lib/features/hideMyEmail/types';
  import SplitPaneSeparator from '$lib/features/splitPane/SplitPaneSeparator.svelte';
  import type { createSplitPane } from '$lib/features/splitPane/createSplitPane.svelte';
  import { onMount } from 'svelte';

  let {
    query,
    splitPane,
    onAliasesChanged = () => {}
  }: {
    query: string;
    splitPane: ReturnType<typeof createSplitPane>;
    onAliasesChanged?: () => void;
  } = $props();

  const copy = $derived(hideMyEmailMessages[accountSession.locale]);
  let status = $state.raw<HideMyEmailStatus>({ connectionState: 'disconnected', setupAvailable: false });
  let aliases = $state.raw<HideMyEmailAliasSummary[]>([]);
  let selectedId = $state('');
  let setupStep = $state<'idle' | 'disclosure' | 'credentials' | 'verification'>('idle');
  let acknowledged = $state(false);
  let verificationMethod = $state<HideMyEmailVerificationMethod>('trustedDevice');
  let verificationMethods = $state.raw<HideMyEmailVerificationMethod[]>(['trustedDevice']);
  let pending = $state(false);
  let error = $state('');
  let notice = $state('');
  let confirmRemove = $state(false);

  const filteredAliases = $derived(
    filterHideMyEmailAliases(
      [...aliases].sort((a, b) => Number(b.isActive) - Number(a.isActive)),
      query
    )
  );
  const selected = $derived(aliases.find((alias) => alias.id === selectedId) ?? aliases[0]);
  const connected = $derived(status.connectionState === 'connected');

  function refreshNotice(result: HideMyEmailRefreshResult) {
    return copy.refreshResult
      .replace('{added}', String(result.added))
      .replace('{updated}', String(result.updated))
      .replace('{unchanged}', String(result.unchanged))
      .replace('{inactive}', String(result.markedInactive));
  }

  async function load() {
    if (accountSession.demoMode) {
      status = {
        connectionState: 'connected',
        setupAvailable: false,
        maskedAppleId: 'd•••@example.com',
        lastRefreshAt: 'Today'
      };
      aliases = demoHideMyEmailAliases;
      selectedId = aliases[0]?.id ?? '';
      return;
    }

    const [statusResult, aliasesResult] = await Promise.allSettled([
      hideMyEmailApi.status(),
      hideMyEmailApi.listAliases()
    ]);
    if (statusResult.status === 'fulfilled') status = statusResult.value;
    if (aliasesResult.status === 'fulfilled') {
      aliases = aliasesResult.value;
      selectedId = aliases[0]?.id ?? '';
    } else {
      error = copy.loadError;
    }
  }

  function beginSetup() {
    error = '';
    notice = '';
    acknowledged = false;
    setupStep = 'disclosure';
  }

  async function startSetup(event: SubmitEvent) {
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    const appleId = String(data.get('appleId') ?? '').trim();
    const passwordInput = form.elements.namedItem('password') as HTMLInputElement;
    const password = passwordInput.value;
    pending = true;
    error = '';

    try {
      const result = await hideMyEmailApi.startSetup(appleId, password);
      passwordInput.value = '';
      if (result.requiresVerification) {
        verificationMethods = result.methods;
        verificationMethod = result.methods[0] ?? 'trustedDevice';
        setupStep = 'verification';
      } else {
        setupStep = 'idle';
        await load();
        onAliasesChanged();
      }
    } catch (cause) {
      passwordInput.value = '';
      const message = String(cause).toLowerCase();
      error = message.includes('security key')
        ? copy.securityKeysUnsupported
        : message.includes('live apple connection')
          ? copy.providerUnavailable
          : copy.actionError;
    } finally {
      pending = false;
    }
  }

  async function verify(event: SubmitEvent) {
    const form = event.currentTarget as HTMLFormElement;
    const input = form.elements.namedItem('code') as HTMLInputElement;
    pending = true;
    error = '';

    try {
      const result = await hideMyEmailApi.verifySetup(input.value.trim(), verificationMethod);
      input.value = '';
      notice = refreshNotice(result);
      setupStep = 'idle';
      await load();
      onAliasesChanged();
    } catch {
      input.value = '';
      error = copy.actionError;
    } finally {
      pending = false;
    }
  }

  async function refresh() {
    pending = true;
    error = '';
    notice = '';
    try {
      const result = await hideMyEmailApi.refresh();
      notice = refreshNotice(result);
      await load();
      onAliasesChanged();
    } catch {
      error = copy.actionError;
    } finally {
      pending = false;
    }
  }

  async function disconnect() {
    pending = true;
    error = '';
    try {
      await hideMyEmailApi.disconnect();
      status = { connectionState: 'disconnected', setupAvailable: false };
      notice = copy.disconnected;
    } catch {
      error = copy.actionError;
    } finally {
      pending = false;
    }
  }

  async function removeAliases() {
    pending = true;
    error = '';
    try {
      await hideMyEmailApi.removeAliases();
      aliases = [];
      selectedId = '';
      confirmRemove = false;
      onAliasesChanged();
    } catch {
      error = copy.actionError;
    } finally {
      pending = false;
    }
  }

  async function copyAlias(alias: HideMyEmailAliasSummary) {
    error = '';
    try {
      if (accountSession.demoMode) {
        await navigator.clipboard.writeText(alias.address);
        notice = copy.demoCopied;
      } else {
        await hideMyEmailApi.copyAlias(alias.id);
        notice = copy.copied;
      }
    } catch {
      error = copy.actionError;
    }
  }

  onMount(() => void load());
</script>

<aside class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))]">
  <header class="border-b border-[rgb(var(--border))] px-5 py-5">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="text-base font-semibold">{copy.section}</h1>
        <p class="mt-1 text-sm text-[rgb(var(--muted))]">{copy.description}</p>
      </div>
      {#if connected && !accountSession.demoMode}
        <Button variant="secondary" disabled={pending} onclick={() => void refresh()}>
          <RefreshCw size={16} class={pending ? 'animate-spin' : ''} />
          {pending ? copy.refreshing : copy.refresh}
        </Button>
      {:else if !accountSession.demoMode}
        <Button onclick={beginSetup}>{status.connectionState === 'expired' ? copy.reconnect : copy.connect}</Button>
      {/if}
    </div>
  </header>

  {#if accountSession.demoMode}
    <div class="m-3 rounded-[var(--radius-md)] bg-[rgb(var(--accent)/0.1)] p-3 text-sm">
      <p class="leading-6">{copy.demoNotice}</p>
      <Button class="mt-3" variant="secondary" onclick={() => void goto('/settings?section=demo')}>
        {copy.exitDemo}
      </Button>
    </div>
  {:else if status.connectionState === 'expired' || (!connected && aliases.length > 0)}
    <p class="m-3 rounded-[var(--radius-md)] bg-[rgb(var(--accent)/0.1)] p-3 text-sm" role="status">
      {status.connectionState === 'expired' ? copy.expired : copy.disconnected}
    </p>
  {/if}

  {#if error}
    <p class="m-3 rounded-[var(--radius-md)] bg-[rgb(var(--danger)/0.1)] p-3 text-sm" role="alert">{error}</p>
  {/if}
  {#if notice}
    <p class="m-3 rounded-[var(--radius-md)] bg-[rgb(var(--accent)/0.1)] p-3 text-sm" role="status">{notice}</p>
  {/if}

  <div class="grid gap-2 p-3">
    {#each filteredAliases as alias (alias.id)}
      <div
        class={[
          'flex min-h-16 items-center gap-2 rounded-[var(--radius-md)] border border-transparent p-2 transition-colors duration-200 hover:border-[rgb(var(--border))] hover:bg-[rgb(var(--surface-muted))]',
          selected?.id === alias.id && 'bg-[rgb(var(--primary)/0.08)]'
        ]}
      >
        <button
          type="button"
          aria-pressed={selected?.id === alias.id}
          class="grid min-w-0 flex-1 cursor-pointer gap-1 rounded-[var(--radius-sm)] p-1 text-left focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
          onclick={() => (selectedId = alias.id)}
        >
          <span class="flex items-center justify-between gap-3">
            <span class="truncate font-medium">{alias.label || alias.address}</span>
            <Badge tone={alias.isActive ? 'accent' : 'neutral'}>
              {alias.isActive ? copy.active : copy.inactive}
            </Badge>
          </span>
          <span class="truncate text-sm text-[rgb(var(--muted))]">{alias.address}</span>
        </button>
        <button
          type="button"
          class="grid h-10 w-10 shrink-0 place-content-center rounded-[var(--radius-sm)] text-[rgb(var(--muted))] hover:bg-[rgb(var(--surface))] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[rgb(var(--ring))]"
          aria-label={`${copy.copy}: ${alias.address}`}
          onclick={() => void copyAlias(alias)}
        >
          <Copy size={16} />
        </button>
      </div>
    {:else}
      {#if aliases.length > 0}
        <p class="p-4 text-sm text-[rgb(var(--muted))]">{copy.noResults}</p>
      {/if}
    {/each}
  </div>
</aside>

<SplitPaneSeparator
  label="Resize vault columns"
  min={splitPane.minLeftPaneWidth}
  max={splitPane.maxLeftPaneWidth}
  value={splitPane.constrainedLeftPaneWidth}
  onpointerdown={splitPane.startResize}
  onkeydown={splitPane.resizeWithKeyboard}
/>

<main class="min-h-[28rem] overflow-auto bg-[rgb(var(--surface))] p-5 md:p-8">
  <div class="mx-auto max-w-3xl">
    {#if setupStep === 'disclosure' || (!connected && aliases.length === 0 && setupStep === 'idle')}
      <div class="max-w-xl">
        <div class="flex h-12 w-12 items-center justify-center rounded-[var(--radius-md)] bg-[rgb(var(--accent)/0.12)]" aria-hidden="true">
          <ShieldAlert size={22} />
        </div>
        <h2 class="mt-5 text-balance text-2xl font-semibold">{copy.setupTitle}</h2>
        <p class="mt-2 text-pretty text-sm leading-6 text-[rgb(var(--muted))]">{copy.setupDescription}</p>
        <p class="mt-2 text-pretty text-sm leading-6">{copy.prerequisites}</p>
        <div class="mt-6 rounded-[var(--radius-lg)] bg-[rgb(var(--surface-muted))] p-5">
          <h3 class="font-semibold">{copy.experimentalTitle}</h3>
          <p class="mt-2 text-pretty text-sm leading-6 text-[rgb(var(--muted))]">{copy.experimentalDescription}</p>
          <p class="mt-3 text-pretty text-sm leading-6">{copy.localOnly}</p>
          {#if !status.setupAvailable}
            <p class="mt-3 text-pretty text-sm font-medium leading-6 text-[rgb(var(--danger))]">{copy.providerUnavailable}</p>
          {/if}
        </div>
        <label class="mt-5 flex min-h-10 items-start gap-3 text-sm leading-6">
          <input class="mt-1 h-4 w-4 accent-[rgb(var(--primary))]" type="checkbox" bind:checked={acknowledged} />
          <span>{copy.acknowledge}</span>
        </label>
        <Button class="mt-4 active:scale-[0.96] transition-transform" disabled={!acknowledged || !status.setupAvailable} onclick={() => (setupStep = 'credentials')}>
          {copy.continue}
        </Button>
      </div>
    {:else if setupStep === 'credentials'}
      <form class="max-w-md" onsubmit={(event) => { event.preventDefault(); void startSetup(event); }}>
        <h2 class="text-balance text-2xl font-semibold">{copy.connect}</h2>
        <label class="mt-6 grid gap-2 text-sm font-medium" for="hme-apple-id">
          {copy.appleId}
          <Input id="hme-apple-id" name="appleId" type="email" autocomplete="username" required />
        </label>
        <label class="mt-4 grid gap-2 text-sm font-medium" for="hme-password">
          {copy.password}
          <Input id="hme-password" name="password" type="password" autocomplete="current-password" required />
        </label>
        <p class="mt-3 text-pretty text-xs leading-5 text-[rgb(var(--muted))]">{copy.localOnly}</p>
        <div class="mt-6 flex gap-3">
          <Button type="button" variant="ghost" disabled={pending} onclick={() => (setupStep = 'disclosure')}>{copy.back}</Button>
          <Button type="submit" disabled={pending}>{pending ? copy.signingIn : copy.continue}</Button>
        </div>
      </form>
    {:else if setupStep === 'verification'}
      <form class="max-w-md" onsubmit={(event) => { event.preventDefault(); void verify(event); }}>
        <h2 class="text-balance text-2xl font-semibold">{copy.verificationTitle}</h2>
        <p class="mt-2 text-pretty text-sm leading-6 text-[rgb(var(--muted))]">{copy.verificationDescription}</p>
        {#if verificationMethods.length > 1}
          <fieldset class="mt-5 flex gap-4">
            <legend class="sr-only">Verification method</legend>
            {#each verificationMethods as method (method)}
              <label class="flex min-h-10 items-center gap-2 text-sm">
                <input type="radio" name="method" value={method} bind:group={verificationMethod} />
                {method === 'sms' ? copy.sms : copy.trustedDevice}
              </label>
            {/each}
          </fieldset>
        {/if}
        <label class="mt-5 grid gap-2 text-sm font-medium" for="hme-code">
          {copy.verificationCode}
          <Input id="hme-code" name="code" inputmode="numeric" pattern="[0-9]{6}" autocomplete="one-time-code" required />
        </label>
        <div class="mt-6 flex gap-3">
          <Button type="button" variant="ghost" disabled={pending} onclick={() => (setupStep = 'credentials')}>{copy.back}</Button>
          <Button type="submit" disabled={pending}>{pending ? copy.verifying : copy.verify}</Button>
        </div>
      </form>
    {:else if selected}
      <div class="flex items-start justify-between gap-5">
        <div class="min-w-0">
          <Badge tone={selected.isActive ? 'accent' : 'neutral'}>{selected.isActive ? copy.active : copy.inactive}</Badge>
          <h2 class="mt-4 truncate text-balance text-2xl font-semibold">{selected.label || selected.address}</h2>
          {#if selected.origin}<p class="mt-2 text-sm text-[rgb(var(--muted))]">{selected.origin}</p>{/if}
        </div>
        <div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-[var(--radius-md)] bg-[rgb(var(--accent)/0.12)]" aria-hidden="true">
          <Mail size={22} />
        </div>
      </div>
      <div class="mt-8 rounded-[var(--radius-lg)] bg-[rgb(var(--surface-muted))] p-5">
        <p class="break-all font-mono text-sm">{selected.address}</p>
        <Button class="mt-4 active:scale-[0.96] transition-transform" variant="secondary" onclick={() => void copyAlias(selected)}>
          <Copy size={16} />{copy.copy}
        </Button>
      </div>
      <div class="mt-8 border-t border-[rgb(var(--border))] pt-6">
        {#if status.lastRefreshAt}
          <p class="text-sm text-[rgb(var(--muted))]">
            {copy.lastRefresh}: <span class="tabular-nums">{status.lastRefreshAt}</span>
          </p>
        {/if}
        {#if !accountSession.demoMode}
          <p class="mt-4 text-sm leading-6 text-[rgb(var(--muted))]">{copy.disconnectHelp}</p>
          <div class="mt-3 flex flex-wrap gap-3">
            {#if connected}
              <Button variant="secondary" disabled={pending} onclick={() => void disconnect()}><Unplug size={16} />{copy.disconnect}</Button>
            {:else}
              <Button onclick={beginSetup}>{copy.reconnect}</Button>
            {/if}
            <Button variant="ghost" disabled={pending} onclick={() => (confirmRemove = true)}>{copy.remove}</Button>
          </div>
        {/if}
      </div>
      {#if confirmRemove}
        <div class="mt-6 rounded-[var(--radius-lg)] bg-[rgb(var(--danger)/0.08)] p-5" role="alertdialog" aria-modal="true" aria-labelledby="remove-aliases-title">
          <h3 id="remove-aliases-title" class="font-semibold">{copy.removeQuestion.replace('{count}', String(aliases.length))}</h3>
          <p class="mt-2 text-sm leading-6 text-[rgb(var(--muted))]">{copy.removeHelp}</p>
          <div class="mt-4 flex gap-3">
            <Button variant="ghost" disabled={pending} onclick={() => (confirmRemove = false)}>{copy.cancel}</Button>
            <Button variant="danger" disabled={pending} onclick={() => void removeAliases()}>{pending ? copy.removing : copy.confirmRemove}</Button>
          </div>
        </div>
      {/if}
    {:else}
      <div class="grid min-h-80 place-content-center text-center">
        <Mail class="mx-auto text-[rgb(var(--muted))]" size={28} />
        <h2 class="mt-4 text-balance text-xl font-semibold">{copy.selectTitle}</h2>
        <p class="mt-2 max-w-sm text-pretty text-sm leading-6 text-[rgb(var(--muted))]">{copy.selectDescription}</p>
      </div>
    {/if}
  </div>
</main>
