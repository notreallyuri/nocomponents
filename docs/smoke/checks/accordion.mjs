import { demo } from '../cdp.mjs';

export default async (t) => {
  const tr = `${demo('[data-slot=accordion-trigger]')}[0]`;
  const s0 = await t.attr(tr, 'aria-expanded');
  await t.click(tr, 450);
  const s1 = await t.attr(tr, 'aria-expanded');
  t.ok('click toggles', s1 !== s0);
  const open = s1 === 'true' ? tr : null;
  if (open) t.ok('an open trigger controls a real element', await t.ev(`!!document.getElementById((${tr}).getAttribute('aria-controls'))`));
  await t.click(tr, 450);
  t.expect('click again restores', await t.attr(tr, 'aria-expanded'), s0);
  await t.focus(tr);
  await t.key('ArrowDown');
  t.expect('ArrowDown moves focus to the next trigger', await t.ev(`document.activeElement.dataset.slot`), 'accordion-trigger');
  t.ok('a different trigger', await t.ev(`document.activeElement !== ${tr}`));
};
