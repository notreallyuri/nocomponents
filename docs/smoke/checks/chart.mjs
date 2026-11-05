import { demo } from '../cdp.mjs';

export default async (t) => {
  const imgs = demo('[role=img]');
  t.ok('charts render', (await t.ev(`${imgs}.length`)) >= 1);
  t.expect('every chart has an accessible name', await t.ev(`${imgs}.every((e) => e.getAttribute('aria-label'))`), true);
  t.ok('series are drawn', (await t.ev(`${demo('[data-slot=chart-series]')}.length`)) >= 1);
  t.expect('no NaN in any drawn path', await t.ev(`${demo('svg path, svg rect, svg circle')}.some((e) => /NaN/.test(e.outerHTML))`), false);
};
