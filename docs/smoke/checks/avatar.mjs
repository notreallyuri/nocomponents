import { demo } from '../cdp.mjs';

export default async (t) => {
  t.ok('fallbacks render', (await t.ev(`${demo('[data-slot=avatar-fallback]')}.length`)) >= 1);
  t.expect('no broken image is left visible', await t.ev(`${demo('img')}.filter((i) => i.complete && i.naturalWidth === 0 && getComputedStyle(i).display !== 'none' && i.getBoundingClientRect().width > 0 && i.closest('[data-slot=avatar]')).length`), 0);
};
