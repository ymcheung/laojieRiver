import type { HideMyEmailAliasSummary } from './types';

export function filterHideMyEmailAliases(aliases: HideMyEmailAliasSummary[], query: string) {
  const needle = query.trim().toLocaleLowerCase();
  if (!needle) return aliases;

  return aliases.filter((alias) =>
    [alias.label, alias.address, alias.origin].some((value) =>
      value?.toLocaleLowerCase().includes(needle)
    )
  );
}
