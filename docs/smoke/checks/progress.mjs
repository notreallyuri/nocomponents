import { demo } from '../cdp.mjs';

export default async (t) => {
  const bars = demo('[role=progressbar]');
  t.ok('progress bars render', (await t.ev(`${bars}.length`)) >= 1);
  t.expect('every bar has an accessible name', await t.ev(`${bars}.every((e) => e.getAttribute('aria-label') || e.getAttribute('aria-labelledby'))`), true);
  t.expect('no bar renders NaN', await t.ev(`${bars}.some((e) => e.innerHTML.includes('NaN') || (e.getAttribute('style') ?? '').includes('NaN'))`), false);
  t.expect('determinate bars report a value', await t.ev(`${bars}.filter((e) => e.hasAttribute('aria-valuenow')).every((e) => !Number.isNaN(Number(e.getAttribute('aria-valuenow'))))`), true);
};
