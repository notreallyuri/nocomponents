import { byText, q, qa } from '../cdp.mjs';

const triggers = `[...document.querySelectorAll('main [data-slot=select-trigger]')]`;

export default async (t) => {
  t.expect('three demo triggers', await t.ev(`${triggers}.map((e) => e.textContent.trim())`), ['Select a fruit...', 'Dark', 'Select a role...']);

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
};
