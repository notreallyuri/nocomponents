import { demo } from '../cdp.mjs';

export default async (t) => {
  const b = `${demo('button')}.find((b) => /^(show|hide)$/i.test(b.textContent.trim()) || /(show|hide) password/i.test(b.getAttribute('aria-label') ?? ''))`;
  t.ok('a show/hide button exists', await t.exists(b));
  const inp = `(${b}).closest('div').querySelector('input')`;
  t.expect('password starts masked', await t.ev(`(${inp}).type`), 'password');
  await t.click(b, 200);
  t.expect('show reveals it', await t.ev(`(${inp}).type`), 'text');
  await t.click(b, 200);
  t.expect('hide masks it again', await t.ev(`(${inp}).type`), 'password');
};
