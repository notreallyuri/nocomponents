import { demo } from '../cdp.mjs';

export default async (t) => {
  const th = `${demo('[role=slider]')}[0]`;
  const v0 = Number(await t.attr(th, 'aria-valuenow'));
  t.ok('has min and max', (await t.attr(th, 'aria-valuemin')) !== null && (await t.attr(th, 'aria-valuemax')) !== null);
  await t.focus(th);
  await t.key('ArrowRight');
  await t.key('ArrowRight');
  const v1 = Number(await t.attr(th, 'aria-valuenow'));
  t.ok(`arrow keys raise the value (${v0} -> ${v1})`, v1 > v0);
  await t.key('ArrowLeft');
  t.expect('ArrowLeft lowers it by one step', Number(await t.attr(th, 'aria-valuenow')) < v1, true);
};
