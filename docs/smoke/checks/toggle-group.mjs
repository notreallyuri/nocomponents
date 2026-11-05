import { demo } from '../cdp.mjs';

export default async (t) => {
  t.ok('a group renders', await t.exists(`${demo('[data-slot=toggle-group]')}[0]`));
  const item = `${demo('[data-slot=toggle-group-item]')}[1]`;
  const before = await t.ev(`${item}.getAttribute('aria-pressed') ?? ${item}.getAttribute('aria-checked')`);
  await t.click(item, 250);
  t.ok('click changes the item', (await t.ev(`${item}.getAttribute('aria-pressed') ?? ${item}.getAttribute('aria-checked')`)) !== before);
};
