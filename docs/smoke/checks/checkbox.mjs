import { demo } from '../cdp.mjs';

export default async (t) => {
  const cb = `${demo('[role=checkbox]')}[0]`;
  const before = await t.attr(cb, 'data-state');
  t.ok('starts in a checked/unchecked state', ['checked', 'unchecked'].includes(before));
  await t.click(cb);
  const after = await t.attr(cb, 'data-state');
  t.ok('click flips data-state', after !== before);
  t.expect('aria-checked follows', await t.attr(cb, 'aria-checked'), after === 'checked' ? 'true' : 'false');
  await t.focus(cb);
  await t.key(' ');
  t.expect('Space flips it back', await t.attr(cb, 'data-state'), before);
  t.expect('no empty ids', await t.ev(`document.querySelectorAll('[id=""]').length`), 0);
  t.expect('every label[for] has a target', await t.ev(`${demo('label[for]')}.filter((l) => !document.getElementById(l.htmlFor)).length`), 0);
};
