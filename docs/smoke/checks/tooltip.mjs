import { demo } from '../cdp.mjs';

export default async (t) => {
  const trig = `${demo('[data-slot=tooltip-trigger]')}[0]`;
  t.ok('a tooltip trigger exists', await t.exists(trig));
  await t.hover(trig, 900);
  t.expect('tooltip content visible', await t.ev(`[...document.querySelectorAll('[data-slot=tooltip-content]')].some((e) => e.getBoundingClientRect().width > 0 && getComputedStyle(e).visibility !== 'hidden')`), true);
  t.expect('role tooltip present', await t.exists(`document.querySelector('[role=tooltip]')`), true);
};
