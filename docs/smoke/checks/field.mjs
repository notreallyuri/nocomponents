import { demo } from '../cdp.mjs';

export default async (t) => {
  t.expect('no empty ids', await t.ev(`document.querySelectorAll('[id=""]').length`), 0);
  t.expect('every label[for] has a target', await t.ev(`${demo('label[for]')}.filter((l) => !document.getElementById(l.htmlFor)).length`), 0);
  t.expect('every field control is named', await t.ev(`${demo('[data-slot=field] input')}.filter((i) => i.type !== 'checkbox' && !i.id && !i.getAttribute('aria-label') && !i.closest('label')).length`), 0);
  t.ok('field errors announce themselves', (await t.ev(`${demo('[data-slot=field-error]')}.every((e) => e.getAttribute('role') === 'alert')`)));
};
