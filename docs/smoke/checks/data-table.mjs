export default async (t) => {
  const rows = `document.querySelectorAll('main table tbody tr').length`;
  t.ok('has rows', (await t.ev(rows)) > 1);

  const cb = `document.querySelector('main table tbody [role=checkbox]')`;
  if (await t.exists(cb)) {
    const bg0 = await t.ev(`getComputedStyle((${cb}).closest('tr')).backgroundColor`);
    await t.click(cb, 300);
    t.expect('selecting marks the row', await t.ev(`(${cb}).closest('tr').dataset.state`), 'selected');
    t.ok('and highlights it', (await t.ev(`getComputedStyle((${cb}).closest('tr')).backgroundColor`)) !== bg0);
    await t.click(cb, 300);
    t.ok('deselecting clears it', (await t.ev(`(${cb}).closest('tr').dataset.state`)) !== 'selected');

    const head = `document.querySelector('main table thead [role=checkbox]')`;
    if (await t.exists(head)) {
      t.expect('nothing selected: the select-all is unchecked', await t.attr(head, 'aria-checked'), 'false');
      await t.click(cb, 300);
      t.expect('some selected: the select-all is mixed', await t.attr(head, 'aria-checked'), 'mixed');
      t.expect('and says so in data-state', await t.attr(head, 'data-state'), 'indeterminate');
      await t.click(head, 300);
      t.expect('clicking a mixed select-all selects the page', await t.attr(head, 'aria-checked'), 'true');
      await t.click(head, 300);
      t.expect('clicking it again clears the page', await t.attr(head, 'aria-checked'), 'false');
    } else t.fail('select-all', 'no header checkbox');
  } else t.fail('selection', 'no row checkbox');

  const tbl = `document.querySelector('main table')`;
  const h = `(${tbl}).querySelector('th[data-slot=table-sort-header] button')`;
  t.ok('a sortable header exists', await t.exists(h));
  const th = `(${h}).closest('th')`;
  const col = `[...(${tbl}).querySelectorAll('thead th')].indexOf(${th}) + 1`;
  const column = `[...(${tbl}).querySelectorAll('tbody tr')].map((r) => r.querySelector('td:nth-child(' + (${col}) + ')')?.textContent.trim())`;
  const sorts = [await t.attr(th, 'aria-sort')];
  const cols = [await t.ev(column)];
  for (let i = 0; i < 3; i++) {
    await t.click(h, 300);
    sorts.push(await t.attr(th, 'aria-sort'));
    cols.push(await t.ev(column));
  }
  t.ok(`clicking cycles aria-sort (${sorts.join(' > ')})`, new Set(sorts).size >= 2);
  const asc = cols[sorts.indexOf('ascending')];
  const desc = cols[sorts.indexOf('descending')];
  if (asc && desc) {
    t.expect('descending is ascending reversed', asc.slice().reverse(), desc);
    t.ok('and the rows actually moved', JSON.stringify(asc) !== JSON.stringify(desc));
  } else t.fail('sort directions', `saw only ${sorts.join(', ')}`);
};
