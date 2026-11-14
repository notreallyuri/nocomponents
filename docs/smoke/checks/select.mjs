import { byText, q, qa } from '../cdp.mjs';
import { assertPageFrozen, lockState } from '../lib.mjs';

const triggers = `[...document.querySelectorAll('main [data-slot=select-trigger]')]`;

export default async (t) => {
  t.expect('four demo triggers', await t.ev(`${triggers}.map((e) => e.textContent.trim())`), ['Select a fruit...', 'Dark', 'Pick a country...', 'Select a role...']);

  const first = `${triggers}[0]`;
  await t.click(first, 400);
  t.expect('aria-expanded', await t.attr(first, 'aria-expanded'), 'true');
  t.expect('options', await t.ev(`[...document.querySelectorAll('[role=option]')].map((o) => o.textContent.trim())`), ['Apple', 'Banana', 'Cherry', 'Durian']);
  t.expect('listbox is capped in height', await t.ev(`getComputedStyle(document.querySelector('[role=listbox]')).maxHeight`), '384px');
  t.expect('the list scrolls inside a scroller', await t.exists(`document.querySelector('[role=listbox] .overflow-y-auto')`), true);

  await t.click(`document.querySelectorAll('[role=option]')[1]`, 400);
  await t.settle('picking updates the trigger', `${first}.textContent.trim()`, 'Banana');
  await t.settle('and closes the list', `${first}.getAttribute('aria-expanded')`, 'false');

  const dark = `${triggers}.find((e) => e.textContent.trim() === 'Dark')`;
  t.ok('default value labels the trigger', await t.exists(dark));

  const phone = `${triggers}[2]`;
  const list = `document.querySelector('[role=listbox]')`;
  await t.viewport(1280, 900);
  await t.click(phone, 400);
  t.expect('at 1280 the country list is floating', await t.ev(`${list}?.dataset.presentation`), 'floating');
  t.expect('and does not lock the page', await t.ev(lockState), [false, '', '']);
  await t.key('Escape', 400);

  await t.viewport(390, 800, true);
  await t.sleep(300);
  await t.click(phone, 500);
  t.expect('at 390 the country list is a sheet', await t.ev(`${list}?.dataset.presentation`), 'sheet');
  t.expect('titled', await t.ev(`document.querySelector('[data-slot=menu-sheet-title]')?.textContent`), 'Country');
  t.expect('all the countries are options', await t.ev(`document.querySelectorAll('[role=option]').length`), 22);
  t.ok('long enough to scroll', await t.ev(`(() => { const s = ${list}.querySelector('.overflow-y-auto'); return s.scrollHeight > s.clientHeight + 40; })()`));
  await assertPageFrozen(t, [195, 60], 'select sheet');
  t.ok('the scroller contains its scroll', await t.ev(`getComputedStyle(${list}.querySelector('.overflow-y-auto')).overscrollBehaviorY === 'contain'`));
  await t.click(`document.querySelectorAll('[role=option]')[3]`, 500);
  await t.settle('picking closes the sheet', `${phone}.getAttribute('aria-expanded')`, 'false');
  t.expect('and unlocks the page', await t.ev(lockState), [false, '', '']);
  t.expect('the trigger shows the pick', await t.ev(`${phone}.textContent.trim()`), 'China');
  await t.viewport(1280, 900);
};
