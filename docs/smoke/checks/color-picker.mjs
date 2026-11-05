import { demo } from '../cdp.mjs';

export default async (t) => {
  const fmt = `${demo('[data-slot=select-trigger]')}.find((b) => /^(hex|rgb|hsl|hsv|oklch)$/i.test(b.textContent.trim()))`;
  t.ok('the format control is a Select', await t.exists(fmt));
  const labels = `[...(${fmt}).closest('section').querySelectorAll('input')].map((i) => i.getAttribute('aria-label'))`;
  t.expect('HEX shows one value field', (await t.ev(labels)).includes('Colour value'), true);
  await t.click(fmt, 400);
  t.expect('offers the formats', await t.ev(`[...document.querySelectorAll('[role=option]')].map((o) => o.textContent.trim())`), ['HEX', 'RGB', 'HSL', 'HSV', 'OKLCH']);
  await t.click(`[...document.querySelectorAll('[role=option]')].find((o) => o.textContent.trim() === 'RGB')`, 400);
  t.expect('choosing updates the trigger', await t.text(fmt), 'RGB');
  const after = await t.ev(labels);
  t.ok('RGB swaps in channel fields', ['red', 'green', 'blue'].every((c) => after.includes(c)));
  t.ok('and drops the hex field', !after.includes('Colour value'));

  await t.click(fmt, 400);
  await t.click(`[...document.querySelectorAll('[role=option]')].find((o) => o.textContent.trim() === 'OKLCH')`, 400);
  t.ok('OKLCH swaps in lightness and chroma', await t.ev(labels).then((l) => l.includes('lightness') && l.includes('chroma')));
};
