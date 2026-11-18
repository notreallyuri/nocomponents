import { section } from '../cdp.mjs';

const items = (sec) => `[...${sec}.querySelectorAll('[data-slot=sortable-item]')]`;
const order = (sec) => `${items(sec)}.map((e) => e.dataset.sortableLabel || e.textContent.trim()).join(',')`;
const item = (sec, label) =>
  `${items(sec)}.find((e) => (e.dataset.sortableLabel || e.textContent.trim()) === ${JSON.stringify(label)})`;
const handle = (sec, label) => `${item(sec, label)}.querySelector('[data-slot=sortable-handle]')`;
const live = `document.querySelector('main [aria-live=assertive]')`;
const liveIn = (sec) => `${sec}.querySelector('[aria-live=assertive]')`;

const drag = async (t, from, dx, dy, { hold = 0, mid } = {}) => {
  const [x, y] = await t.center(from);
  await t.mouse('mouseMoved', x, y);
  await t.mouse('mousePressed', x, y);
  const steps = Math.max(8, Math.ceil(Math.hypot(dx, dy) / 12));
  for (let i = 1; i <= steps; i++) {
    await t.mouse('mouseMoved', x + (dx * i) / steps, y + (dy * i) / steps);
    await t.sleep(16);
  }
  if (hold) await t.sleep(hold);
  if (mid) await mid();
  await t.mouse('mouseReleased', x + dx, y + dy);
  await t.sleep(250);
};

export default async (t) => {
  const basic = section(/^Default/);

  t.expect('starts in order', await t.ev(order(basic)), 'Sketch,Prototype,Review,Ship');
  t.ok('handles are named', (await t.attr(handle(basic, 'Sketch'), 'aria-label')) === 'Reorder Sketch');
  t.ok('handles are described', !!(await t.attr(handle(basic, 'Sketch'), 'aria-describedby')));
  t.expect('handles carry the role description', await t.attr(handle(basic, 'Sketch'), 'aria-roledescription'), 'sortable');

  await t.focus(handle(basic, 'Sketch'));
  await t.key(' ');
  t.ok('picking up is announced', /Picked up Sketch, position 1 of 4/.test(await t.ev(`${liveIn(basic)}.textContent`)));
  t.expect('the root says a move is under way', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), 'true');
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  t.ok('a move is announced', /Sketch moved to position 3 of 4/.test(await t.ev(`${liveIn(basic)}.textContent`)));
  t.expect('nothing is applied until the drop', await t.ev(order(basic)), 'Sketch,Prototype,Review,Ship');
  await t.key(' ');
  await t.settle('Space drops it there', order(basic), 'Prototype,Review,Sketch,Ship');
  t.ok('the drop is announced', /Dropped Sketch at position 3 of 4/.test(await t.ev(`${liveIn(basic)}.textContent`)));
  t.ok('the move is reported', /Moved Sketch to 2/.test(await t.ev(`document.querySelector('[data-testid=sortable-last]').textContent`)));
  await t.settle('focus stays on the moved item', `document.activeElement === ${handle(basic, 'Sketch')}`, true);
  t.expect('the root is quiet again', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), null);

  await t.focus(handle(basic, 'Ship'));
  await t.key('Enter');
  await t.key('ArrowUp');
  await t.key('Home');
  await t.key('Escape');
  t.expect('Escape restores the order', await t.ev(order(basic)), 'Prototype,Review,Sketch,Ship');
  t.ok('the cancel is announced', /cancelled/i.test(await t.ev(`${liveIn(basic)}.textContent`)));
  t.expect('and ends the move', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), null);

  await t.focus(handle(basic, 'Review'));
  await t.key(' ');
  await t.key('End');
  await t.key('Tab');
  t.expect('losing focus cancels', await t.ev(order(basic)), 'Prototype,Review,Sketch,Ship');
  t.expect('and ends the move', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), null);

  const pitch = await t.ev(`(() => { const [a, b] = ${items(basic)}; return b.getBoundingClientRect().top - a.getBoundingClientRect().top; })()`);
  await drag(t, handle(basic, 'Prototype'), 0, pitch * 2.2, {
    mid: async () => {
      t.expect('the root says a pointer move is under way', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), 'true');
      t.expect('the held item is marked', await t.attr(item(basic, 'Prototype'), 'data-following'), 'true');
      t.ok('others slide aside', (await t.ev(`${items(basic)}.filter((e) => e.style.transform).length`)) >= 3);
    },
  });
  t.expect('a slow pointer drag moves the item', await t.ev(order(basic)), 'Review,Sketch,Prototype,Ship');
  t.expect('the transforms are cleared on the drop', await t.ev(`${items(basic)}.filter((e) => e.style.transform).length`), 0);

  const [hx, hy] = await t.center(handle(basic, 'Ship'));
  await t.clickAt(hx, hy);
  t.expect('a click on a handle moves nothing', await t.ev(order(basic)), 'Review,Sketch,Prototype,Ship');

  const keep = await t.ev(order(basic));
  const [sx, sy] = await t.center(handle(basic, 'Ship'));
  await t.mouse('mouseMoved', sx, sy);
  await t.mouse('mousePressed', sx, sy);
  for (let i = 1; i <= 8; i++) {
    await t.mouse('mouseMoved', sx, sy - i * 16);
    await t.sleep(16);
  }
  t.expect('a pointer drag is under way', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), 'true');
  await t.key('Escape');
  t.expect('Escape ends it', await t.attr(`${basic}.querySelector('[data-slot=sortable]')`, 'data-dragging'), null);
  await t.mouse('mouseMoved', sx, sy - 160);
  await t.mouse('mouseReleased', sx, sy - 160);
  await t.sleep(200);
  t.expect('and the release changes nothing', await t.ev(order(basic)), keep);
  t.expect('nothing is left sliding', await t.ev(`${items(basic)}.filter((e) => e.style.transform).length`), 0);

  const block = section(/^Block drag/);
  t.expect('block list starts in order', await t.ev(order(block)), 'Cover,Front,Back,Index,Notes');
  const tick = (label) => `${items(block)}.find((e) => e.dataset.sortableLabel === ${JSON.stringify(label)}).querySelector('[role=checkbox]')`;
  await t.click(tick('Front'));
  await t.click(tick('Index'));
  t.expect('ticked items are marked', await t.ev(`${items(block)}.filter((e) => e.dataset.selected).map((e) => e.dataset.sortableLabel).join(',')`), 'Front,Index');

  const bpitch = await t.ev(`(() => { const [a, b] = ${items(block)}; return b.getBoundingClientRect().top - a.getBoundingClientRect().top; })()`);
  await drag(t, handle(block, 'Front'), 0, bpitch * 3.3, {
    mid: async () => {
      t.expect('the held item counts the block', await t.ev(`${items(block)}.find((e) => e.dataset.anchor).dataset.count`), '2');
      t.expect('both ride the pointer', await t.ev(`${items(block)}.filter((e) => e.dataset.following).map((e) => e.dataset.sortableLabel).join(',')`), 'Front,Index');
    },
  });
  t.expect('a pointer drag moves the selection as one block', await t.ev(order(block)), 'Cover,Back,Notes,Front,Index');

  await t.focus(handle(block, 'Index'));
  await t.key(' ');
  t.ok('a block pick-up is announced as one', /Picked up 2 items/.test(await t.ev(`${liveIn(block)}.textContent`)));
  await t.key('Home');
  await t.key(' ');
  await t.settle('the keyboard moves the block too', order(block), 'Front,Index,Cover,Back,Notes');

  const rows = section(/^Horizontal/);
  t.expect('tags start in order', await t.ev(order(rows)), 'rust,leptos,wasm,tailwind');
  t.expect('without a handle the item takes focus', await t.attr(`${items(rows)}[0]`, 'tabindex'), '0');
  await t.focus(`${items(rows)}[0]`);
  await t.key(' ');
  await t.key('ArrowDown');
  t.ok('the cross-axis arrows do nothing', /Picked up rust, position 1 of 4/.test(await t.ev(`${liveIn(rows)}.textContent`)));
  await t.key('ArrowRight');
  await t.key(' ');
  await t.settle('Right moves along a horizontal list', order(rows), 'leptos,rust,wasm,tailwind');

  await t.click(`${rows}.querySelector('[role=switch]')`);
  t.expect('disabling takes items out of the tab order', await t.attr(`${items(rows)}[0]`, 'tabindex'), null);
  t.expect('and says so', await t.attr(`${items(rows)}[0]`, 'aria-disabled'), 'true');
  await t.ev(`${items(rows)}[0].focus()`);
  await t.key(' ');
  t.expect('a disabled list cannot be picked up', await t.attr(`${rows}.querySelector('[data-slot=sortable]')`, 'data-dragging'), null);

  const queue = section(/^Scrolling/);
  const list = `${queue}.querySelector('[data-slot=sortable]')`;
  t.ok('the list scrolls', (await t.ev(`${list}.scrollHeight > ${list}.clientHeight`)) === true);
  await t.ev(`${list}.scrollIntoView({ block: 'center' })`);
  await t.center(handle(queue, 'Track 01'));
  const page0 = await t.ev(`window.scrollY`);
  const box = await t.ev(`(() => { const r = ${list}.getBoundingClientRect(); return [r.top, r.bottom]; })()`);
  const [, hy0] = await t.center(handle(queue, 'Track 01'));
  await drag(t, handle(queue, 'Track 01'), 0, box[1] - hy0 - 4, {
    hold: 900,
    mid: async () => {
      t.ok('holding at the edge scrolls the list', (await t.ev(`${list}.scrollTop`)) > 20);
      t.expect('and not the page', await t.ev(`window.scrollY`), page0);
    },
  });
  const queued = await t.ev(order(queue));
  t.ok(`the item lands far down (${queued})`, queued.split(',').indexOf('Track 01') >= 6);

  await windowed(t);
  await windowedBlock(t);
};

const SHIFT = 8;

const windowed = async (t) => {
  const win = section(/^Windowed/);
  const list = `${win}.querySelector('[data-slot=sortable-window]')`;
  const rows = `[...${win}.querySelectorAll('[data-slot=sortable-window-item]')]`;
  const row = (i) => `${win}.querySelector('[data-index="${i}"]')`;
  const grip = (i) => `${row(i)}.querySelector('[role=button]')`;
  const indices = `${rows}.map((e) => +e.dataset.index)`;
  const name = (i) => `${row(i)}.textContent.match(/Track \\d+/)[0]`;
  const last = `document.querySelector('[data-testid=sortable-window-last]').textContent`;
  const held = `document.activeElement.closest('[data-slot=sortable-window-item]')?.dataset.index`;

  await t.ev(`${list}.scrollIntoView({ block: 'center' })`);
  const mounted = await t.ev(`${rows}.length`);
  t.ok(`5,000 rows keep only a few in the page (${mounted})`, mounted >= 8 && mounted < 40);
  t.expect('the first row is the first', await t.ev(name(0)), 'Track 0001');
  t.expect('rows say how many there are', await t.attr(row(0), 'aria-setsize'), '5000');
  t.expect('and which one they are', await t.attr(row(3), 'aria-posinset'), '4');
  t.expect('only one row has the tab stop', await t.ev(`${rows}.filter((e) => e.querySelector('[role=button]').tabIndex === 0).length`), 1);
  t.ok('the list is as tall as every row', (await t.ev(`${list}.scrollHeight`)) >= 5000 * 44);

  await t.focus(grip(0));
  await t.key(' ');
  t.ok('picking up is announced from data', /Picked up Track 0001, position 1 of 5000/.test(await t.ev(`${win}.querySelector('[aria-live=assertive]').textContent`)));
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  t.ok('a move is announced', /Track 0001 moved to position 4 of 5000/.test(await t.ev(`${win}.querySelector('[aria-live=assertive]').textContent`)));
  t.expect('nothing is applied until the drop', await t.ev(name(0)), 'Track 0001');
  t.expect('the root says a move is under way', await t.attr(list, 'data-dragging'), 'true');
  await t.key(' ');
  const firstNames = `[0, 1, 2, 3].map((i) => ${win}.querySelector('[data-index="' + i + '"]').textContent.match(/Track \\d+/)[0]).join(',')`;
  await t.settle('Space drops it three places down', firstNames, 'Track 0002,Track 0003,Track 0004,Track 0001');
  t.ok('the move is reported by index', /Moved 1 from 0 to 3/.test(await t.ev(last)));
  await t.settle('focus follows the moved row', held, '3');
  t.expect('the root is quiet again', await t.attr(list, 'data-dragging'), null);

  await t.focus(grip(3));
  await t.key(' ');
  await t.key('ArrowUp');
  await t.key('Escape');
  t.expect('Escape puts it back', await t.ev(name(3)), 'Track 0001');
  t.ok('and says so', /cancelled/i.test(await t.ev(`${win}.querySelector('[aria-live=assertive]').textContent`)));

  await t.ev(`${list}.scrollTop = 44 * 2000`);
  await t.settle('scrolling renders the rows now in view', `${indices}.filter((i) => i > 1980).length >= 8`, true);
  const deep = await t.ev(indices);
  t.ok(
    `and not the ones that left, bar the row with the tab stop (${deep.length} rows, ${deep[0]}..${deep.at(-1)})`,
    deep.length < 40 && deep.filter((i) => i < 1980).length <= 1 && !deep.includes(0),
  );
  t.expect('the row there is the one at that index', await t.ev(name(2000)), 'Track 2001');

  await t.focus(grip(2000));
  await t.key('End');
  await t.settle('End focuses the last row, which was not in the page', held, '4999');
  t.ok('having scrolled it in', (await t.ev(`${list}.scrollTop`)) > 44 * 4900);
  t.expect('still a few rows', (await t.ev(`${rows}.length`)) < 40, true);
  await t.key('Home');
  await t.settle('Home focuses the first', held, '0');
  await t.key('ArrowDown');
  await t.settle('an arrow moves focus by one', held, '1');
  t.expect('the tab stop moved with it', await t.ev(`${row(1)}.querySelector('[role=button]').tabIndex`), 0);

  await t.ev(`${list}.scrollTop = 44 * 3000`);
  await t.settle('scrolled deep again', `${row(3004)} !== null`, true);
  const before = await t.ev(name(3004));
  const [x, y] = await t.center(grip(3004));
  const started = Date.now();
  await t.mouse('mouseMoved', x, y);
  await t.mouse('mousePressed', x, y);
  for (let i = 1; i <= 30; i++) await t.mouse('mouseMoved', x, y + i * 4);
  const took = Date.now() - started;
  console.log(`  windowed: pick up + 30 moves over 5,000 rows took ${took}ms`);
  t.ok(`a pick-up and 30 moves are quick (${took}ms)`, took < 3000);
  t.expect('the held row follows the pointer', await t.attr(row(3004), 'data-following'), 'true');
  const inPage = await t.ev(`(async () => {
    const t0 = performance.now();
    for (let i = 0; i < 200; i++) {
      const y = ${y} + 120 + (i % 2 ? 30 : -30) * ((i % 7) / 7);
      window.dispatchEvent(new PointerEvent('pointermove', { clientX: ${x}, clientY: y, pointerId: 1, pointerType: 'mouse', bubbles: true }));
      await Promise.resolve();
    }
    return performance.now() - t0;
  })()`);
  console.log(`  windowed: 200 synthetic moves, handlers and restyling included, took ${inPage.toFixed(1)}ms in the page`);
  t.ok(`a move costs the rows in view, not the list (${(inPage / 200).toFixed(2)}ms each)`, inPage / 200 < 5);
  t.ok('while the page stays small', (await t.ev(`${rows}.length`)) < 40);
  await t.mouse('mouseMoved', x, y + 44 * 3 + 4);
  await t.sleep(120);
  await t.mouse('mouseReleased', x, y + 44 * 3 + 4);
  await t.settle('the pointer drop is reported', last, `Moved ${before.replace('Track ', '').replace(/^0+/, '')} from 3004 to 3007`);
  t.expect('and applied', await t.ev(name(3007)), before);
  t.expect('nothing is left riding the pointer', await t.ev(`${rows}.filter((e) => e.dataset.following).length`), 0);

  const [ex, ey] = await t.center(grip(3007));
  const bottom = await t.ev(`${list}.getBoundingClientRect().bottom`);
  const top0 = await t.ev(`${list}.scrollTop`);
  const max0 = await t.ev(`Math.max(...${indices})`);
  await t.mouse('mouseMoved', ex, ey);
  await t.mouse('mousePressed', ex, ey);
  for (let i = 1; i <= 8; i++) {
    await t.mouse('mouseMoved', ex, ey + ((bottom - 4 - ey) * i) / 8);
    await t.sleep(16);
  }
  await t.sleep(700);
  t.ok('holding at the edge scrolls the list', (await t.ev(`${list}.scrollTop`)) > top0 + 100);
  t.ok('and the window follows it', (await t.ev(`Math.max(...${indices})`)) > max0 + 3 && (await t.ev(`${rows}.length`)) < 40);
  t.expect('the held row stays in the page', await t.ev(`${rows}.some((e) => e.dataset.following)`), true);
  await t.key('Escape');
  t.expect('Escape ends it', await t.attr(list, 'data-dragging'), null);
  await t.mouse('mouseReleased', ex, bottom - 4);
  await t.sleep(200);
  t.ok('and the release changes nothing', /from 3004 to 3007/.test(await t.ev(last)));
};

const windowedBlock = async (t) => {
  const win = section(/^Windowed/);
  const list = `${win}.querySelector('[data-slot=sortable-window]')`;
  const row = (i) => `${win}.querySelector('[data-index="${i}"]')`;
  const grip = (i) => `${row(i)}.querySelector('[role=button]')`;
  const tick = (i) => `${row(i)}.querySelector('[role=checkbox]')`;
  const rows = `[...${win}.querySelectorAll('[data-slot=sortable-window-item]')]`;
  const names = (n) => `[...Array(${n}).keys()].map((i) => ${win}.querySelector('[data-index="' + i + '"]')?.textContent.match(/Track \\d+/)?.[0]).join(',')`;
  const last = `document.querySelector('[data-testid=sortable-window-last]').textContent`;
  const count = `document.querySelector('[data-testid=sortable-window-count]').textContent`;
  const live = `${win}.querySelector('[aria-live=assertive]').textContent`;
  const lay = (moved, rest, to) => [...rest.slice(0, to), ...moved, ...rest.slice(to)];

  await t.ev(`${list}.scrollTop = 0`);
  await t.settle('back at the top', `!!${row(0)}`, true);
  t.expect('no checkboxes until Select is on', await t.ev(`${rows}.filter((e) => e.querySelector('[role=checkbox]')).length`), 0);
  await t.click(`${win}.querySelector('[role=switch]')`);
  await t.settle('Select puts a checkbox in every row', `${rows}.every((e) => e.querySelector('[role=checkbox]'))`, true);
  t.ok('and it is the size of the box asked for', await t.ev(`(() => { const b = ${tick(0)}.getBoundingClientRect(); return Math.round(b.width) === 20 && Math.round(b.height) === 20; })()`));
  await t.click(tick(1));
  await t.click(tick(3), 250, SHIFT);
  t.ok('with the tick drawn to fit it', await t.ev(`(() => { const i = ${tick(1)}.querySelector('[data-slot=checkbox-indicator] svg'); if (!i) return false; const r = i.getBoundingClientRect(); return r.width >= 12 && r.width <= 20; })()`));
  t.expect('click then Shift-click selects the run', await t.ev(count), '3 selected');
  t.expect('the rows are marked selected', await t.ev(`${rows}.filter((e) => e.dataset.selected).map((e) => e.dataset.index).join(',')`), '1,2,3');

  const before = (await t.ev(names(12))).split(',');
  const moved = before.slice(1, 4);
  const rest = [before[0], ...before.slice(4)];
  const pitch = 44;

  await drag(t, grip(2), 0, pitch * 3 + 10, {
    mid: async () => {
      t.expect('the held row counts the block', await t.attr(`${win}.querySelector('[data-anchor]')`, 'data-count'), '3');
      t.expect('and the whole selection rides the pointer', await t.ev(`${rows}.filter((e) => e.dataset.following).map((e) => e.dataset.index).join(',')`), '1,2,3');
      t.ok('the others close up around the gap', (await t.ev(`${rows}.filter((e) => e.style.transform).length`)) >= 8);
    },
  });
  t.expect('a pointer drag lands the block as one run', await t.ev(names(10)), lay(moved, rest, 3).slice(0, 10).join(','));
  t.ok('the move is reported as a block', /Moved 3 items to 3/.test(await t.ev(last)));
  t.expect('the selection is kept', await t.ev(count), '3 selected');
  t.expect('nothing is left riding the pointer', await t.ev(`${rows}.filter((e) => e.dataset.following || e.dataset.count).length`), 0);

  await t.focus(grip(4));
  await t.key(' ');
  t.ok('a block pick-up is announced as one', /Picked up 3 items/.test(await t.ev(live)));
  await t.key('Home');
  t.expect('the block sits at the top while it is held', await t.attr(row(4), 'data-placeholder'), 'true');
  await t.key(' ');
  const second = lay(moved, rest, 0);
  await t.settle('the keyboard moves the block too', names(10), second.slice(0, 10).join(','));
  t.ok('and says where the block went', /Dropped 3 items/.test(await t.ev(live)));
  t.ok('reports it', /Moved 3 items to 0/.test(await t.ev(last)));

  await t.ev(`${list}.scrollTop = 44 * 2000`);
  await t.settle('deep in the list', `!!${row(2000)}`, true);
  const far = (await t.ev(`${row(2000)}.textContent.match(/Track \\d+/)[0]`));
  await t.click(tick(2000));
  t.expect('a row far away joins the selection', await t.ev(count), '4 selected');
  await t.ev(`${list}.scrollTop = 0`);
  await t.settle('and back at the top', `!!${row(0)} && !${row(2000)}`, true);
  t.expect('the far row is not in the page', await t.ev(`${rows}.some((e) => e.textContent.includes(${JSON.stringify(far)}))`), false);

  await drag(t, grip(0), 0, pitch * 3 + 10);
  const after = (await t.ev(names(8))).split(',');
  t.expect('a drag takes the far row along too, in list order', after.slice(1, 5), [...moved, far]);
  t.expect('and the rest keep their order', [after[0], ...after.slice(5)], [second[3], ...second.slice(4, 7)]);
  t.ok('it is reported with the block size', /Moved 4 items to 1/.test(await t.ev(last)));
};
