import { demo } from '../cdp.mjs';

export default async (t) => {
  const triggers = `${demo('button')}.filter((b) => /pick a date/i.test(b.textContent))`;
  t.ok('a date-picker trigger exists', await t.exists(`${triggers}[0]`));
  // Re-find by stable means: once a date is chosen the trigger's text no longer says "Pick a date".
  await t.ev(`${triggers}[0].dataset.smoke = 'dp'`);
  const trig = `document.querySelector('[data-smoke=dp]')`;
  await t.click(trig, 500);
  t.expect('aria-expanded', await t.attr(trig, 'aria-expanded'), 'true');
  t.ok('calendar grid opens', await t.exists(`document.querySelector('[role=grid]')`));
  await t.click(`document.querySelector('[role=grid] td:not([aria-disabled=true]) button')`, 500);
  const text = await t.text(trig);
  t.ok(`trigger shows the chosen date (${text})`, text !== null && !/pick a date/i.test(text) && /\d/.test(text));
  await t.settle('popover closes after picking', `${trig}.getAttribute('aria-expanded')`, 'false');
};
