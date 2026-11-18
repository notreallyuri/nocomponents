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

  await range(t);
};

const SHIFT = 8;

// The rule: the anchor is the row clicked last, and a Shift-click sets every row from the anchor to
// the clicked one (both included) to the state the clicked row just took.
const range = async (t) => {
  const sec = section(/^Range selection/);
  const box = (n) => `${sec}.querySelector('[aria-label="Row ${n}"]')`;
  const marked = `[...${sec}.querySelectorAll('[role=checkbox]')].map((b, i) => (b.getAttribute('aria-checked') === 'true' ? i + 1 : 0)).filter(Boolean).join(',')`;
  const count = `document.querySelector('[data-testid=range-count]').textContent`;

  t.expect('range demo starts empty', await t.ev(marked), '');
  await t.click(box(2));
  t.expect('a plain click ticks one row', await t.ev(marked), '2');
  await t.click(box(6), 250, SHIFT);
  t.expect('Shift-click ticks everything from the anchor to the row', await t.ev(marked), '2,3,4,5,6');
  t.expect('and the readout counts them', await t.ev(count), '5 selected');
  // The anchor is the row clicked last (6), and the clicked row's new state is "off".
  await t.click(box(4), 250, SHIFT);
  t.expect('Shift-clicking a ticked row clears the run back to the last click', await t.ev(marked), '2,3');
  await t.click(box(8));
  await t.click(box(7), 250, SHIFT);
  t.expect('a plain click moves the anchor, and Shift reaches only back to it', await t.ev(marked), '2,3,7,8');

  await t.focus(box(1));
  await t.key(' ');
  t.expect('Space ticks a focused box', await t.ev(marked), '1,2,3,7,8');
  await t.focus(box(5));
  await t.key(' ', 200, SHIFT);
  t.expect('Shift and Space extends the range from the keyboard', await t.ev(marked), '1,2,3,4,5,7,8');
};
