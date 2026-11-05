import { dialogTrigger } from '../lib.mjs';

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
};
