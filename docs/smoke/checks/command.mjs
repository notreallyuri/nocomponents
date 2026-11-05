import { section } from '../cdp.mjs';

const shown = (sec) => `${sec}.querySelectorAll('[role=option]')`;
const names = (sec) => `[...${shown(sec)}].filter((o) => o.offsetParent !== null).map((o) => o.textContent.trim())`;

export default async (t) => {
  const first = section(/default/i);
  const inp = `(${first}).querySelector('input')`;
  const before = await t.ev(names(first));
  t.ok('the default demo lists several commands', before.length >= 5);

  await t.focus(inp);
  await t.type('sign');
  await t.settle('typing filters the list', names(first), ['Sign out']);

  await t.ev(`(() => { const i = ${inp}; i.select(); })()`);
  await t.key('Backspace');
  await t.settle('clearing restores the list', `(${names(first)}).length`, before.length);

  const kw = section(/keywords/i);
  await t.focus(`(${kw}).querySelector('input')`);
  await t.type('logout');
  t.ok('keywords match without being visible text', (await t.ev(names(kw))).includes('Sign out'));
};
