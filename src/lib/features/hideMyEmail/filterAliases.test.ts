import assert from 'node:assert/strict';
import test from 'node:test';
import { filterHideMyEmailAliases } from './filterAliases.ts';

const aliases = [
  {
    id: '1',
    address: 'quiet.river@icloud.com',
    label: 'GitHub',
    origin: 'github.com',
    isActive: true
  },
  {
    id: '2',
    address: 'hidden.moon@icloud.com',
    label: 'Newsletter',
    isActive: false
  }
];

test('searches safe alias fields without case sensitivity', () => {
  assert.deepEqual(filterHideMyEmailAliases(aliases, 'GITHUB'), [aliases[0]]);
  assert.deepEqual(filterHideMyEmailAliases(aliases, 'hidden.moon'), [aliases[1]]);
  assert.deepEqual(filterHideMyEmailAliases(aliases, ' newsletter '), [aliases[1]]);
});

test('returns all aliases for an empty query', () => {
  assert.deepEqual(filterHideMyEmailAliases(aliases, '  '), aliases);
});
