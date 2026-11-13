import { demo, section } from '../cdp.mjs';

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

  const sec = section(/indeterminate/i);
  const boxes = `[...${sec}.querySelectorAll('[role=checkbox]')]`;
  const parent = `${boxes}[0]`;
  const states = `${boxes}.map((b) => b.getAttribute('aria-checked'))`;
  t.expect('some children on: the parent is mixed', await t.attr(parent, 'aria-checked'), 'mixed');
  t.expect('and wears data-state=indeterminate', await t.attr(parent, 'data-state'), 'indeterminate');
  t.ok('with the dash drawn and the tick hidden', await t.ev(`(() => { const p = ${parent}; const shown = (s) => getComputedStyle(p.querySelector(s)).display !== 'none'; return shown('[data-slot=checkbox-indeterminate-indicator]') && !shown('[data-slot=checkbox-indicator]'); })()`));
  await t.click(parent, 300);
  t.expect('clicking a mixed box checks it, and the children follow', await t.ev(states), ['true', 'true', 'true', 'true']);
  await t.click(parent, 300);
  t.expect('clicking it again clears them', await t.ev(states), ['false', 'false', 'false', 'false']);
  await t.click(`${boxes}[1]`, 300);
  t.expect('one child on makes the parent mixed again', await t.attr(parent, 'aria-checked'), 'mixed');
};
