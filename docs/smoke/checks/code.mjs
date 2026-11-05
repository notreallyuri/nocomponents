import { demo } from '../cdp.mjs';

export default async (t) => {
  t.ok('code blocks render', (await t.ev(`${demo('pre')}.length`)) >= 1);
  t.ok('with text in them', (await t.ev(`${demo('pre')}[0].textContent.length`)) > 10);
  t.ok('a copy button exists (the success path needs a clipboard, which headless lacks)', await t.exists(`${demo('[data-slot=code-copy]')}[0]`));
};
