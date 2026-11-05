import { demo } from '../cdp.mjs';

export default async (t) => {
  const sw = `${demo('[role=switch]')}[0]`;
  const thumb = `(${sw}).querySelector('span')`;
  const x = () => t.ev(`Math.round(${thumb}.getBoundingClientRect().x - ${sw}.getBoundingClientRect().x)`);
  const s0 = await t.attr(sw, 'data-state');
  const x0 = await x();
  await t.click(sw, 450);
  t.ok('data-state flips', (await t.attr(sw, 'data-state')) !== s0);
  t.expect('aria-checked follows', await t.attr(sw, 'aria-checked'), s0 === 'checked' ? 'false' : 'true');
  t.ok('thumb moves', (await x()) !== x0);
  t.expect('no empty ids', await t.ev(`document.querySelectorAll('[id=""]').length`), 0);
  t.expect('every label[for] has a target', await t.ev(`${demo('label[for]')}.filter((l) => !document.getElementById(l.htmlFor)).length`), 0);
};
