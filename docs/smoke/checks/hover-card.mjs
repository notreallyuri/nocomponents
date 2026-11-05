import { demo } from '../cdp.mjs';

export default async (t) => {
  const trig = `${demo('[data-slot=hover-card-trigger]')}[0]`;
  t.ok('a hover-card trigger exists', await t.exists(trig));
  await t.hover(trig, 1200);
  t.expect('content visible', await t.ev(`[...document.querySelectorAll('[data-slot=hover-card-content]')].some((e) => e.getBoundingClientRect().width > 0 && getComputedStyle(e).visibility !== 'hidden')`), true);
};
