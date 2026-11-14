import { demoBtn } from '../cdp.mjs';
import { assertPageFrozen, dialogTrigger, dragBy, lockState } from '../lib.mjs';

export default async (t) => {
  const trig = dialogTrigger();
  t.ok('a dialog trigger exists', await t.exists(trig));
  t.expect('trigger slot', await t.attr(trig, 'data-slot'), 'dialog-trigger');
  t.expect('trigger starts collapsed', await t.attr(trig, 'aria-expanded'), 'false');

  await t.click(trig, 500);
  const info = await t.ev(`(() => {
    const d = document.querySelector('[role=dialog]');
    const e = ${trig};
    return d && { id: d.id, controls: e.getAttribute('aria-controls'), expanded: e.getAttribute('aria-expanded'), inside: d.contains(document.activeElement), modal: d.getAttribute('aria-modal') };
  })()`);
  t.ok('dialog is in the document', !!info);
  if (info) {
    t.ok('dialog has an id', info.id !== '');
    t.expect('aria-controls points at the dialog', info.controls, info.id);
    t.expect('aria-expanded', info.expanded, 'true');
    t.expect('focus moved inside', info.inside, true);
  }

  await t.key('Escape', 500);
  t.expect('Escape collapses the trigger', await t.attr(trig, 'aria-expanded'), 'false');
  t.expect('focus returns to the trigger', await t.ev(`document.activeElement === ${trig}`), true);
  await t.settle('dialog leaves the DOM or hides', `(() => { const d = document.querySelector('[role=dialog]'); return !d || d.closest('[data-state]')?.dataset.state === 'closed' || getComputedStyle(d).visibility === 'hidden'; })()`, true);

  // On phones, with snap points: a sheet that opens at 40 percent and rests at 90.
  const panel = `document.querySelector('[role=dialog]')`;
  const header = `document.querySelector('[role=dialog] [data-slot=menu-sheet-header]')`;
  const snap = `${panel}?.dataset.snap ?? null`;
  const filters = demoBtn('Open filters');
  const closedNow = `(!${panel} || ${panel}.dataset.state === 'closed')`;
  const shown = async () => t.ev(`innerHeight - ${panel}.getBoundingClientRect().top`);

  await t.viewport(390, 800, true);
  await t.sleep(300);
  await t.click(filters, 600);
  t.expect('a snapping dialog is a sheet at 390', await t.ev(`${panel}.dataset.presentation`), 'sheet');
  t.expect('it opens at the first snap point', await t.ev(snap), '0');
  const first = await shown();
  t.ok('which is 40 percent of the screen', Math.abs(first - 320) < 24, String(first));
  t.ok('the header has a close button and the corner one is hidden', await t.ev(`!!${panel}.querySelector('[data-slot=menu-sheet-close]') && getComputedStyle(${panel}.querySelector('[data-slot=dialog-close]')).display === 'none'`));

  await dragBy(t, header, -280);
  t.expect('a drag up settles on the second snap point', await t.ev(snap), '1');
  const second = await shown();
  t.ok('which is 90 percent of the screen', Math.abs(second - 720) < 24, String(second));

  await dragBy(t, header, 40);
  t.expect('a short drag stays where it was', await t.ev(snap), '1');

  await dragBy(t, header, 700);
  await t.settle('a drag below the lowest point dismisses it', closedNow, true);

  await t.click(filters, 600);
  t.expect('it reopens at the first point', await t.ev(snap), '0');
  await t.click(`${panel}.querySelector('[data-slot=menu-sheet-close]')`, 400);
  await t.settle('the header close button closes it', closedNow, true);
  t.expect('closing the dialog unlocks the page', await t.ev(lockState), [false, '', '']);

  await t.click(filters, 600);
  await assertPageFrozen(t, [195, 40], 'dialog sheet');
  await t.key('Escape', 500);
  await t.settle('Escape closes it', closedNow, true);
  t.expect('and unlocks the page', await t.ev(lockState), [false, '', '']);

  await t.viewport(1280, 900);
};
