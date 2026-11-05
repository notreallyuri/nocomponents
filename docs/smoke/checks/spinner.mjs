import { demo } from '../cdp.mjs';

export default async (t) => {
  const s = demo('[role=status]');
  t.ok('spinners render', (await t.ev(`${s}.length`)) >= 1);
  t.expect('every spinner is named', await t.ev(`${s}.every((e) => e.getAttribute('aria-label'))`), true);
};
