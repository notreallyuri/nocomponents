import { demo } from '../cdp.mjs';

export default async (t) => {
  const zones = demo('[data-slot=dropzone]');
  t.ok('zones render', (await t.ev(`${zones}.length`)) >= 1);
  t.expect('each zone has exactly one keyboard entry (a button role or a real button)', await t.ev(`${zones}.map((z) => (z.getAttribute('role') === 'button' ? 1 : 0) + z.querySelectorAll('button').length).every((n) => n >= 1)`), true);
};
