import { qa } from '../cdp.mjs';

export default async (t) => {
  t.ok('a grid exists', await t.exists(`document.querySelector('main [role=grid]')`));
  t.expect('exactly one tab stop in every grid', await t.ev(`[...document.querySelectorAll('main [role=grid]')].map((g) => g.querySelectorAll('button[tabindex="0"], [tabindex="0"]').length).every((n) => n === 1)`), true);

  const day = `document.querySelector('main [role=grid] td:not([aria-disabled=true]) button')`;
  await t.click(day, 300);
  t.expect('clicking a day selects it', await t.ev(`(${day}).closest('td').getAttribute('aria-selected')`), 'true');

  const dis = `document.querySelector('main td[aria-disabled=true] button')`;
  if (await t.exists(dis)) {
    const before = await t.ev(`document.querySelectorAll('td[aria-selected=true]').length`);
    await t.click(dis, 300);
    t.expect('a disabled day cannot be selected', await t.ev(`document.querySelectorAll('td[aria-selected=true]').length`), before);
  }
};
