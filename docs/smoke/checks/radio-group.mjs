import { demo } from '../cdp.mjs';

export default async (t) => {
  const items = `${demo('[role=radio]')}`;
  t.ok('radios render', (await t.ev(`${items}.length`)) >= 2);
  await t.click(`${items}[1]`, 300);
  t.expect('clicking checks the second radio', await t.ev(`${items}.slice(0, 3).map((e) => e.getAttribute('aria-checked'))`).then((a) => a.indexOf('true')), 1);
  t.expect('only one is checked within the group', await t.ev(`${demo('[role=radiogroup]')}[0].querySelectorAll('[role=radio][aria-checked=true]').length`), 1);
  await t.key('ArrowDown');
  t.expect('arrow moves the selection', await t.ev(`document.activeElement.getAttribute('role')`), 'radio');
};
