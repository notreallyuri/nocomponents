import { demo, section } from '../cdp.mjs';

export default async (t) => {
  t.expect('no empty ids', await t.ev(`document.querySelectorAll('[id=""]').length`), 0);
  t.expect('every label[for] has a target', await t.ev(`${demo('label[for]')}.filter((l) => !document.getElementById(l.htmlFor)).length`), 0);
  t.expect('every field control is named', await t.ev(`${demo('[data-slot=field] input')}.filter((i) => i.type !== 'checkbox' && i.type !== 'hidden' && i.getAttribute('aria-hidden') !== 'true' && !i.id && !i.getAttribute('aria-label') && !i.closest('label')).length`), 0);
  t.ok('field errors announce themselves', (await t.ev(`${demo('[data-slot=field-error]')}.every((e) => e.getAttribute('role') === 'alert')`)));

  const form = `${section(/native form/i)}.querySelector('form')`;
  const pick = (css) => `${form}.querySelector(${JSON.stringify(css)})`;
  const output = () => t.text(pick('[data-form-output]'));
  const submit = `[...${form}.querySelectorAll('button')].find((b) => b.textContent.trim() === 'Submit')`;
  const reset = `[...${form}.querySelectorAll('button')].find((b) => b.textContent.trim() === 'Reset')`;
  const terms = pick('[role=checkbox]');
  const beta = pick('[role=switch]');
  const radios = `[...${form}.querySelectorAll('[role=radio]')]`;
  const trigger = pick('[data-slot=select-trigger]');
  const thumb = pick('[role=slider]');
  const entries = `[...new FormData(${form}).entries()].map(([k, v]) => k + '=' + v)`;

  t.expect('the slider wears the id its field label points at', await t.ev(`document.getElementById([...${form}.querySelectorAll('label')].find((l) => l.textContent.trim() === 'Volume').htmlFor)?.getAttribute('role')`), 'slider');

  const shadows = `[...${form}.querySelectorAll('input[aria-hidden=true]')]`;
  t.expect('two radios, a switch and a checkbox keep a native input', await t.ev(`${shadows}.map((i) => i.type)`), ['radio', 'radio', 'checkbox', 'checkbox']);
  t.expect('four shadow inputs', await t.ev(`${shadows}.length`), 4);
  t.ok('shadow inputs are not tab stops', await t.ev(`${shadows}.every((i) => i.tabIndex === -1)`));
  t.ok('shadow inputs take no pointer events', await t.ev(`${shadows}.every((i) => getComputedStyle(i).pointerEvents === 'none' && getComputedStyle(i).opacity === '0')`));
  t.expect('the select is aria-hidden and not a tab stop', await t.ev(`(() => { const s = ${pick('select[aria-hidden=true]')}; return [s.tabIndex, s.name]; })()`), [-1, 'plan']);
  t.expect('no shadow outside a form or a name', await t.ev(`document.querySelectorAll('main input[aria-hidden=true]').length`), 4);

  await t.focus(beta);
  await t.key('Tab');
  t.expect('Tab from the switch lands on the checkbox, not a shadow', await t.ev(`document.activeElement === ${terms}`), true);
  await t.key('Tab');
  t.expect('and on from the checkbox to Submit', await t.ev(`document.activeElement === ${submit}`), true);

  t.expect('defaults submit what is checked', await t.ev(entries), ['project=', 'notes=', 'region=eu', 'plan=free', 'volume=30', 'cadence=weekly']);

  await t.click(submit);
  t.expect('an empty required input blocks the submit', await output(), '');
  await t.click(pick('input[name=project]'));
  await t.type('apple');
  await t.click(submit);
  t.expect('an unticked required checkbox blocks it too', await output(), '');
  t.expect('the form reports it invalid', await t.ev(`${form}.checkValidity()`), false);

  await t.click(terms);
  t.expect('ticking it sets the shadow', await t.ev(`${shadows}.at(-1).checked`), true);
  await t.click(submit);
  t.expect('submitting lists the entries; the unchecked switch is absent', (await output()).split('\n'), ['project=apple', 'notes=', 'region=eu', 'plan=free', 'volume=30', 'cadence=weekly', 'terms=accepted']);

  await t.click(beta);
  await t.click(radios + '[0]');
  await t.click(trigger, 400);
  await t.click(`[...document.querySelectorAll('[role=option]')].find((o) => o.textContent.trim() === 'Pro')`, 400);
  await t.settle('the select trigger shows the pick', `${trigger}.textContent.trim()`, 'Pro');
  await t.focus(thumb);
  await t.key('End');
  await t.click(pick('textarea'));
  await t.type('hello');
  await t.ev(`(() => { const s = ${pick('select[name=region]')}; s.value = 'us'; s.dispatchEvent(new Event('change', { bubbles: true })); })()`);
  await t.click(submit);
  t.expect('every change reaches the data', (await output()).split('\n'), ['project=apple', 'notes=hello', 'region=us', 'plan=pro', 'volume=100', 'cadence=daily', 'beta=on', 'terms=accepted']);

  await t.click(reset);
  await t.settle('reset unticks the checkbox', `${terms}.getAttribute('data-state')`, 'unchecked');
  t.expect('and switches the switch off', await t.attr(beta, 'data-state'), 'unchecked');
  t.expect('and returns the radio group to its default', await t.ev(`${radios}.map((r) => r.dataset.state)`), ['unchecked', 'checked']);
  t.expect('and the select to its default', await t.text(trigger), 'Free');
  t.expect('and the slider to its default', await t.attr(thumb, 'aria-valuenow'), '30');
  t.expect('and clears the output', await output(), '');
  t.expect('and the form data agrees', await t.ev(entries), ['project=', 'notes=', 'region=eu', 'plan=free', 'volume=30', 'cadence=weekly']);
};
