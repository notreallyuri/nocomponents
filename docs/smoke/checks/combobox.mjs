import { demo } from '../cdp.mjs';

export default async (t) => {
  const trig = `${demo('[data-slot=combobox-trigger]')}[0]`;
  t.expect('trigger starts with its placeholder', await t.text(trig), 'Select a framework…');
  await t.click(trig, 500);
  t.expect('aria-expanded', await t.attr(trig, 'aria-expanded'), 'true');
  t.expect('the search input takes focus', await t.ev(`document.activeElement.tagName + '|' + document.activeElement.placeholder`), 'INPUT|Search framework…');
  t.ok('options listed', (await t.ev(`document.querySelectorAll('[role=option]').length`)) >= 3);
  await t.click(`document.querySelectorAll('[role=option]')[1]`, 500);
  t.expect('picking labels the trigger', await t.text(trig), 'SvelteKit');
  t.expect('and closes the list', await t.attr(trig, 'aria-expanded'), 'false');
};
