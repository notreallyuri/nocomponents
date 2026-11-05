export default async (t) => {
  t.ok('tree items exist', await t.exists(`document.querySelector('main [role=treeitem]')`));
  t.expect('the first tree starts expanded on its first branch', await t.ev(`document.querySelector('main [role=treeitem]').getAttribute('aria-expanded')`), 'true');

  // The tree keeps no selection of its own; the Selection demo writes "Selected: <name>".
  const status = `[...document.querySelectorAll('main p')].map((e) => e.textContent.trim()).find((x) => /^Selected: /.test(x))`;
  const leaf = (name) => `[...document.querySelectorAll('main [role=treeitem]')].filter((e) => e.textContent.trim() === ${JSON.stringify(name)})[name === 'lib.rs' ? 1 : 0]`;
  t.expect('initial selection', await t.ev(status), 'Selected: lib.rs');
  await t.click(leaf('main.rs'), 300);
  await t.settle('on_select reports the clicked row', status, 'Selected: main.rs');
  await t.click(leaf('tree.rs'), 300);
  await t.settle('and the next', status, 'Selected: tree.rs');
};
