import { demo } from '../cdp.mjs';

export default async (t) => {
  const inp = demo('input');
  t.ok('an input exists', (await t.ev(`${inp}.length`)) >= 1);
  // The demos leave `label` unset, and the primitive then writes no aria-label at all; an empty one is the bug.
  t.expect('no input carries an empty aria-label', await t.ev(`${inp}.some((i) => i.getAttribute('aria-label') === '')`), false);
  t.expect('no empty ids', await t.ev(`document.querySelectorAll('[id=""]').length`), 0);
  const first = `${inp}[0]`;
  await t.focus(first);
  await t.type('123', 300);
  t.ok('typing fills the slots', await t.ev(`${demo('[data-slot=input-otp-slot]')}.slice(0, 3).every((s) => /\\d/.test(s.textContent))`));
};
