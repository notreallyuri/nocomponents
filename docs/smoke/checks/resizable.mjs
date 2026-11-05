import { demo } from '../cdp.mjs';

export default async (t) => {
  const h = `${demo('[role=separator]')}[0]`;
  const p = `${demo('[data-slot=resizable-panel]')}[0]`;
  const w = () => t.ev(`Math.round(${p}.getBoundingClientRect().width)`);
  const w0 = await w();
  await t.focus(h);
  await t.key('ArrowRight');
  await t.key('ArrowRight');
  t.ok(`arrow keys resize the panel (${w0} -> ${await w()})`, (await w()) !== w0);
  t.ok('handle reports a value', (await t.attr(h, 'aria-valuenow')) !== null);
};
