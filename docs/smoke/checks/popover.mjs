import { demo, section, qa } from '../cdp.mjs';

const own = `${demo('[data-slot=popover-trigger]')}`;

export default async (t) => {
  const trig = `${own}[0]`;
  await t.click(trig, 500);
  t.expect('aria-expanded', await t.attr(trig, 'aria-expanded'), 'true');
  t.expect('content slot', await t.exists(`document.querySelector('[data-slot=popover-content]')`), true);
  t.expect('opening moves focus into the content', await t.ev(`document.querySelector('[data-slot=popover-content]').contains(document.activeElement)`), true);
  await t.clickAt(1150, 860, 600);
  t.expect('outside click closes', await t.attr(trig, 'aria-expanded'), 'false');
  t.expect('a pointer close does not pull focus back onto the trigger', await t.ev(`document.activeElement === ${trig}`), false);

  await t.focus(trig);
  await t.key('Enter', 500);
  t.expect('Enter opens it', await t.attr(trig, 'aria-expanded'), 'true');
  await t.key('Escape', 500);
  t.expect('Escape closes', await t.attr(trig, 'aria-expanded'), 'false');
  t.expect('focus is back on the trigger', await t.ev(`document.activeElement === ${trig}`), true);

  // Focus return follows the input that closed it: an input inside the panel has focus either way.
  const formTrig = `${section(/form/i)}.querySelector('[data-slot=popover-trigger]')`;
  const inner = `document.querySelector('[data-slot=popover-content] input')`;
  await t.click(formTrig, 500);
  if (!(await t.exists(inner))) return t.fail('form popover', 'no input inside the panel');
  t.expect('focus lands on the first focusable element inside', await t.ev(`document.activeElement === ${inner}`), true);
  await t.focus(inner);
  await t.clickAt(1150, 860, 600);
  t.expect('form popover closed by an outside click', await t.attr(formTrig, 'aria-expanded'), 'false');
  t.expect('mouse close: focus is not pulled onto the trigger', await t.ev(`document.activeElement === ${formTrig}`), false);
  await t.click(formTrig, 500);
  await t.focus(inner);
  await t.key('Escape', 500);
  t.expect('Escape closes the form popover', await t.attr(formTrig, 'aria-expanded'), 'false');
  t.expect('keyboard close: focus is back on the trigger', await t.ev(`document.activeElement === ${formTrig}`), true);

  // The reactive-side demo: a button cycles the placement, the next open uses it.
  const sec = section(/reactive/i);
  const trig2 = `[...${sec}.querySelectorAll('[data-slot=popover-trigger]')].pop()`;
  const cycle = `[...${sec}.querySelectorAll('button')].find((b) => b.dataset.slot !== 'popover-trigger' && !/code|copy/i.test(b.textContent))`;
  if (!(await t.exists(cycle))) return t.fail('reactive demo', 'no cycle button found');
  const sides = [];
  for (let i = 0; i < 3; i++) {
    await t.click(trig2, 500);
    sides.push(await t.ev(`document.querySelector('[data-slot=popover-content]')?.dataset.side ?? null`));
    await t.key('Escape', 400);
    await t.click(cycle, 300);
  }
  t.ok('side is always reported', sides.every(Boolean));
  t.ok('cycling the signal changes the placement', new Set(sides).size >= 2);
};
