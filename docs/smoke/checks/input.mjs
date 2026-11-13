import { demo, section } from '../cdp.mjs';

export default async (t) => {
  const b = `${demo('button')}.find((b) => /^(show|hide)$/i.test(b.textContent.trim()) || /(show|hide) password/i.test(b.getAttribute('aria-label') ?? ''))`;
  t.ok('a show/hide button exists', await t.exists(b));
  const inp = `(${b}).closest('div').querySelector('input')`;
  t.expect('password starts masked', await t.ev(`(${inp}).type`), 'password');
  await t.click(b, 200);
  t.expect('show reveals it', await t.ev(`(${inp}).type`), 'text');
  await t.click(b, 200);
  t.expect('hide masks it again', await t.ev(`(${inp}).type`), 'password');

  const form = `${section(/resetting a form/i)}.querySelector('form')`;
  const out = () => t.text(`${form}.querySelector('[data-reset-output]')`);
  const fields = `[${form}.querySelector('input'), ${form}.querySelector('textarea'), ${form}.querySelector('select')]`;
  const reset = `${form}.querySelector('button[type=reset]')`;
  t.expect('the model starts as the fields', await out(), 'Ada / Mathematician / editor');
  await t.click(`${form}.querySelector('input')`);
  await t.type('x');
  await t.click(`${form}.querySelector('textarea')`);
  await t.type('y');
  await t.ev(`(() => { const s = ${form}.querySelector('select'); s.value = 'admin'; s.dispatchEvent(new Event('change', { bubbles: true })); })()`);
  t.expect('typing reaches the models', await out(), 'Adax / Mathematiciany / admin');
  await t.click(reset, 300);
  await t.settle('a reset writes the defaults back into the models', `${form}.querySelector('[data-reset-output]').textContent.trim()`, 'Ada / Mathematician / editor');
  t.expect('and the screen agrees', await t.ev(`${fields}.map((f) => f.value)`), ['Ada', 'Mathematician', 'editor']);
  await t.click(reset, 300);
  t.expect('a second reset with nothing to undo leaves them be', await t.ev(`${fields}.map((f) => f.value)`), ['Ada', 'Mathematician', 'editor']);
  const load = `[...${form}.querySelectorAll('button')].find((b) => b.textContent.trim() === 'Load profile')`;
  await t.click(load, 300);
  await t.click(`${form}.querySelector('input')`);
  await t.type('z');
  t.expect('a profile loaded later is in the model', (await out()).split(' / ')[0], 'Gracez');
  await t.click(reset, 300);
  await t.settle('a reset returns to the default as it is now, not as it was at mount', `${form}.querySelector('[data-reset-output]').textContent.trim().split(' / ')[0]`, 'Grace');
  t.expect('and the field shows it', await t.ev(`${form}.querySelector('input').value`), 'Grace');
};
