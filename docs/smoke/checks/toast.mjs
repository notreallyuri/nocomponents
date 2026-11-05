import { demoBtn } from '../cdp.mjs';

const live = `[...document.querySelectorAll('[role=status],[role=alert]')].map((t) => t.getAttribute('role') + '/' + t.getAttribute('aria-live'))`;

export default async (t) => {
  await t.click(demoBtn('Default'), 700);
  const a = await t.ev(live);
  t.ok('a toast appears with a live role', a.length >= 1);
  t.ok('a default toast is polite', a.some((s) => s.endsWith('/polite') || s === 'status/null'));
  await t.click(demoBtn('Destructive'), 700);
  const b = await t.ev(live);
  t.ok('a second toast stacks', b.length > a.length);
  t.ok('a destructive toast is announced assertively', b.some((s) => s.startsWith('alert') || s.endsWith('/assertive')));
  t.ok('toasts live in a labelled Notifications region', await t.ev(`document.querySelectorAll('[role=region][aria-label=Notifications]').length`) >= 1);
};
