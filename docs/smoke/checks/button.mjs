import { demo, section } from '../cdp.mjs';

const icon = (sec, label) =>
  `[...${sec}.querySelectorAll('button')].find((b) => b.textContent.trim() === ${JSON.stringify(label)}).querySelector('svg')`;
const side = async (t, sel) => Math.round(await t.ev(`${sel}.getBoundingClientRect().width`));

export default async (t) => {
  const sec = section(/^Icons$/);
  const xs = await side(t, icon(sec, 'Xs'));
  const sm = await side(t, icon(sec, 'Sm'));
  const def = await side(t, icon(sec, 'Default'));
  const lg = await side(t, icon(sec, 'Lg'));

  t.expect('icon in a default button is 16px', def, 16);
  t.expect('icon in an Xs button is 12px', xs, 12);
  t.ok('icon in an Xs button is smaller than in a default one', xs < def);
  t.ok('sizes step up: xs < sm <= default <= lg', xs < sm && sm <= def && def <= lg);

  t.expect('an icon with its own size-6 inside a small button is 24px', await side(t, icon(sec, 'Own size')), 24);

  t.ok('the page has demo buttons', (await t.ev(`${demo('button')}.length`)) > 0);
};
