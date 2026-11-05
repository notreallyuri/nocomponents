import { dialogTrigger } from '../lib.mjs';

export default async (t) => {
  const trig = dialogTrigger();
  t.ok('trigger exists', await t.exists(trig));
  await t.click(trig, 500);
  t.expect('role is alertdialog', await t.ev(`document.querySelector('[role=alertdialog]') ? 'alertdialog' : document.querySelector('[role=dialog]') ? 'dialog' : 'none'`), 'alertdialog');
  t.expect('aria-expanded', await t.attr(trig, 'aria-expanded'), 'true');
  t.expect('focus inside', await t.ev(`!!document.activeElement.closest('[role=alertdialog]')`), true);
  const action = `[...document.querySelectorAll('[role=alertdialog] button')].pop()`;
  t.ok('an action button exists', await t.exists(action));
  await t.click(action, 600);
  t.expect('the action closes the dialog', await t.attr(trig, 'aria-expanded'), 'false');
};
