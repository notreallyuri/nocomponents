import { qa } from '../cdp.mjs';

export default async (t) => {
  const tabs = `[...document.querySelectorAll('main [role=tab]')]`;
  const sel = `${tabs}.find((e) => e.getAttribute('aria-selected') === 'true')`;
  t.ok('tabs exist', await t.exists(`${tabs}[0]`));
  t.expect('selected tab owns a tabpanel labelled by it', await t.ev(`(() => {
    const t = ${sel}; const p = document.getElementById(t.getAttribute('aria-controls'));
    return [t.id !== '', p?.getAttribute('role'), p?.getAttribute('aria-labelledby') === t.id];
  })()`), [true, 'tabpanel', true]);
  t.expect('every tab has an id and aria-controls', await t.ev(`${tabs}.every((e) => e.id && e.getAttribute('aria-controls'))`), true);

  await t.click(`${tabs}[1]`);
  t.expect('clicking selects the second tab', await t.ev(`${tabs}.slice(0, 3).map((e) => e.getAttribute('aria-selected'))`).then((a) => a.indexOf('true')), 1);

  const o = await t.ev(`${sel}.closest('[role=tablist]').getAttribute('aria-orientation')`);
  const next = o === 'vertical' ? 'ArrowDown' : 'ArrowRight';
  await t.focus(sel);
  const label = await t.text(sel);
  await t.key(next);
  t.ok('an arrow key moves focus to another tab', (await t.ev(`document.activeElement.textContent.trim()`)) !== label);
  t.expect('focus is on a tab', await t.ev(`document.activeElement.getAttribute('role')`), 'tab');
};
