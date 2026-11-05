import { demo } from '../cdp.mjs';

export default async (t) => {
  const s = `${demo('select')}[0]`;
  t.ok('a native select renders', await t.exists(s));
  const opts = await t.ev(`[...(${s}).options].map((o) => o.value)`);
  t.ok('with options', opts.length >= 2);
  await t.ev(`(() => { const s = ${s}; s.value = s.options[s.options.length - 1].value; s.dispatchEvent(new Event('change', { bubbles: true })); })()`);
  t.expect('value follows', await t.ev(`(${s}).value`), opts[opts.length - 1]);
};
