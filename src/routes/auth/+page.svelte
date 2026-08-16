<script lang="ts">
  import { goto } from '$app/navigation';
  import { KeyRound, Languages, Mail } from '@lucide/svelte';
  import Button from '$lib/components/ui/button/Button.svelte';
  import Input from '$lib/components/ui/input/Input.svelte';
  import { messages, type Locale } from '$lib/features/account/i18n';
  import { accountSession } from '$lib/features/account/session.svelte';

  let email = $state(accountSession.pendingEmail);
  let code = $state('');
  let submitting = $state(false);
  const copy = $derived(messages[accountSession.locale]);
  const enteringCode = $derived(accountSession.pendingEmail.length > 0);

  async function requestCode() {
    submitting = true;
    try {
      await accountSession.requestOtp(email);
    } catch {
      // The session seam owns the privacy-safe user message.
    } finally {
      submitting = false;
    }
  }

  async function verifyCode() {
    submitting = true;
    try {
      await accountSession.verifyOtp(code);
      await goto(accountSession.realVaultDestination);
    } catch {
      // The session seam owns the privacy-safe user message.
    } finally {
      submitting = false;
    }
  }

  async function startDemo() {
    await accountSession.enterDemo();
    await goto('/vault');
  }

  function changeEmail() {
    accountSession.cancelOtp();
    code = '';
  }
</script>

<main class="grid min-h-screen place-items-center px-6 py-10">
  <section
    class="w-full max-w-md rounded-[var(--radius-lg)] bg-[rgb(var(--surface))] p-6 shadow-[0_1px_2px_rgb(0_0_0/0.06),0_12px_36px_rgb(0_0_0/0.08)]"
  >
    <div
      class="flex h-11 w-11 items-center justify-center rounded-[var(--radius-md)] bg-[rgb(var(--surface-muted))] text-[rgb(var(--accent-foreground))]"
      aria-hidden="true"
    >
      {#if enteringCode}<KeyRound size={20} />{:else}<Mail size={20} />{/if}
    </div>

    <h1 class="mt-5 text-2xl font-semibold text-balance">
      {enteringCode ? copy.codeTitle : copy.authTitle}
    </h1>
    <p class="mt-2 text-sm leading-6 text-pretty text-[rgb(var(--muted))]">
      {enteringCode ? copy.codeDescription : copy.authDescription}
    </p>

    <form
      class="mt-6 grid gap-4"
      onsubmit={(event) => {
        event.preventDefault();
        void (enteringCode ? verifyCode() : requestCode());
      }}
    >
      {#if enteringCode}
        <label class="grid gap-2 text-sm font-medium" for="otp">
          {copy.code}
          <Input
            id="otp"
            bind:value={code}
            autocomplete="one-time-code"
            inputmode="numeric"
            pattern="[0-9]*"
            required
          />
        </label>
      {:else}
        <label class="grid gap-2 text-sm font-medium" for="email">
          {copy.email}
          <Input id="email" bind:value={email} autocomplete="email" inputmode="email" type="email" required />
        </label>
      {/if}

      {#if accountSession.error}
        <p
          class="rounded-[var(--radius-md)] bg-[rgb(var(--danger)/0.1)] px-3 py-2 text-sm text-[rgb(var(--foreground))]"
          role="alert"
        >
          {accountSession.error}
        </p>
        {#if accountSession.status === 'error'}
          <Button variant="secondary" type="button" onclick={() => void accountSession.initialize()}>
            {copy.retry}
          </Button>
        {/if}
      {/if}

      {#if accountSession.notice}
        <p class="rounded-[var(--radius-md)] bg-[rgb(var(--surface-muted))] px-3 py-2 text-sm">
          {accountSession.notice}
        </p>
        <Button variant="secondary" type="button" onclick={() => void accountSession.initialize()}>
          {copy.retry}
        </Button>
      {/if}

      <Button class="w-full active:scale-[0.96] transition-transform" disabled={submitting} type="submit">
        {enteringCode
          ? submitting
            ? copy.verifying
            : copy.verify
          : submitting
            ? copy.sendingCode
            : copy.sendCode}
      </Button>
    </form>

    <div class="mt-4 flex min-h-10 flex-wrap items-center justify-between gap-2">
      {#if enteringCode}
        <button
          class="min-h-10 text-sm font-medium text-[rgb(var(--accent-foreground))]"
          type="button"
          onclick={changeEmail}
        >
          {copy.changeEmail}
        </button>
        <button
          class="min-h-10 text-sm font-medium text-[rgb(var(--accent-foreground))]"
          type="button"
          disabled={submitting}
          onclick={() => void requestCode()}
        >
          {copy.resend}
        </button>
      {:else}
        <button
          class="min-h-10 text-sm font-medium text-[rgb(var(--muted))]"
          type="button"
          onclick={() => void startDemo()}
        >
          {copy.tryDemo}
        </button>
      {/if}

      {#if enteringCode && (accountSession.status === 'error' || accountSession.pendingLogout)}
        <button
          class="min-h-10 text-sm font-medium text-[rgb(var(--muted))]"
          type="button"
          onclick={() => void startDemo()}
        >
          {copy.tryDemo}
        </button>
      {/if}
    </div>

    <label class="mt-5 flex min-h-10 items-center gap-2 text-sm text-[rgb(var(--muted))]" for="locale">
      <Languages size={16} aria-hidden="true" />
      {copy.language}
      <select
        id="locale"
        class="ml-auto min-h-10 rounded-[var(--radius-md)] bg-[rgb(var(--surface-muted))] px-3 text-[rgb(var(--foreground))]"
        value={accountSession.locale}
        onchange={(event) => accountSession.setLocale(event.currentTarget.value as Locale)}
      >
        <option value="en">{copy.english}</option>
        <option value="tw">{copy.chinese}</option>
      </select>
    </label>
  </section>
</main>
