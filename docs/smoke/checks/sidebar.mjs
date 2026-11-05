export default async (t) => {
  await t.viewport(390, 800, true);
  await t.goto('/docs/sidebar', { settle: 800 });
  const trig = `document.querySelector('[data-slot=sidebar-trigger]')`;
  t.ok('a sidebar trigger exists', await t.exists(trig));
  await t.click(trig, 700);
  const id = await t.attr(trig, 'aria-controls');
  t.ok('aria-controls is set', !!id);
  t.expect('and names an existing element', await t.ev(`!!document.getElementById(${JSON.stringify(id)})`), true);
  t.expect('the mobile sidebar opens as a sheet/dialog', await t.ev(`!!document.querySelector('[role=dialog]')`), true);
};
