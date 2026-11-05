import { demo, qa } from '../cdp.mjs';

export default async (t) => {
  const roots = `${demo('[data-slot=collapsible]')}`;
  const states = await t.ev(`${roots}.map((e) => e.dataset.state)`);
  t.ok('at least one collapsible', states.length > 0);
  t.ok('one is mounted open (default_open)', states.includes('open'));
  const trig = `${demo('[data-slot=collapsible-trigger]')}[0]`;
  const open0 = await t.attr(trig, 'aria-expanded');
  await t.click(trig, 400);
  t.ok('trigger toggles aria-expanded', (await t.attr(trig, 'aria-expanded')) !== open0);
};
