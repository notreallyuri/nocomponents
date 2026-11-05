import { demo } from '../cdp.mjs';

export default async (t) => {
  const b = `${demo('[data-slot=toggle]')}[0]`;
  const s0 = await t.attr(b, 'aria-pressed');
  await t.click(b, 250);
  t.ok('click flips aria-pressed', (await t.attr(b, 'aria-pressed')) !== s0);
  t.expect('data-state follows', await t.attr(b, 'data-state'), (await t.attr(b, 'aria-pressed')) === 'true' ? 'on' : 'off');
};
