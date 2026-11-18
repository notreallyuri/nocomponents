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
  t.expect('the progress says where it would land', await t.ev(`${basic}.querySelector('[aria-live=polite]').textContent`), 'Moving Sketch. Destination position 3 of 4.');
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
      t.expect('only the held one rides the pointer', await t.ev(`${items(block)}.filter((e) => e.dataset.following).map((e) => e.dataset.sortableLabel).join(',')`), 'Front');
      t.expect('the other is folded into it', await t.ev(`${items(block)}.filter((e) => e.dataset.hidden).map((e) => e.dataset.sortableLabel).join(',')`), 'Index');
      t.expect('and is not drawn', await t.ev(`getComputedStyle(${items(block)}.find((e) => e.dataset.sortableLabel === 'Index')).visibility`), 'hidden');
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

  await domBlock(t);
  await sideways(t);

  await windowed(t);
  await windowedBlock(t);
  await t.goto('/docs/sortable');
  await windowedTen(t);
};

const probe = (sec, axis) => `(() => {
  const its = [...${sec}.querySelectorAll('[data-slot=sortable-item]')];
  const [from, to] = ${JSON.stringify(axis)} === 'x' ? ['left', 'right'] : ['top', 'bottom'];
  const info = its.map((e) => {
    const r = e.getBoundingClientRect();
    return { label: e.dataset.sortableLabel, hidden: getComputedStyle(e).visibility === 'hidden', flagged: !!e.dataset.hidden, anchor: !!e.dataset.anchor, placeholder: !!e.dataset.placeholder, start: r[from], end: r[to] };
  });
  const rest = info.filter((i) => !i.hidden && !i.anchor).sort((a, b) => a.start - b.start);
  const anchor = info.find((i) => i.anchor);
  return {
    hidden: info.filter((i) => i.hidden).map((i) => i.label).join(','),
    flagged: info.filter((i) => i.flagged).map((i) => i.label).join(','),
    placeholders: info.filter((i) => i.placeholder).length,
    count: its.find((e) => e.dataset.anchor)?.dataset.count,
    size: anchor ? anchor.end - anchor.start : 0,
    gaps: rest.slice(1).map((i, k) => i.start - rest[k].end),
    rest: rest.map((i) => i.label).join(','),
    loose: its.filter((e) => e.style.transform || e.style.visibility).length,
  };
})()`;

const inOrder = (all, picked) => {
  const keep = all.filter((l) => !picked.includes(l));
  return { keep, block: picked };
};

const checkCollapsed = (t, what, seen, pitchGap, hidden, held) => {
  t.expect(`${what}: the other selected items are hidden`, seen.hidden, hidden.join(','));
  t.expect(`${what}: and flagged`, seen.flagged, hidden.join(','));
  t.expect(`${what}: the held item counts the block`, String(seen.count), String(hidden.length + 1));
  t.expect(`${what}: nothing is a ring placeholder`, seen.placeholders, 0);
  const slot = seen.size + pitchGap;
  const wide = seen.gaps.filter((g) => Math.abs(g - pitchGap) > 2.5);
  t.ok(`${what}: the items that stay are contiguous but for the one slot (${seen.gaps.map((g) => Math.round(g)).join(',')})`, wide.length <= 1 && wide.every((g) => Math.abs(g - pitchGap - slot) <= 2.5));
  t.ok(`${what}: the held item is not among them`, !seen.rest.split(',').includes(held));
};

const domBlock = async (t) => {
  const sec = section(/^Scrolling/);
  const list = `${sec}.querySelector('[data-slot=sortable]')`;
  const its = `[...${sec}.querySelectorAll('[data-slot=sortable-item]')]`;
  const order = `${its}.map((e) => e.dataset.sortableLabel).join(',')`;
  const grip = (label) => `${its}.find((e) => e.dataset.sortableLabel === ${JSON.stringify(label)}).querySelector('[data-slot=sortable-handle]')`;
  const tick = (label) => `${its}.find((e) => e.dataset.sortableLabel === ${JSON.stringify(label)}).querySelector('[role=checkbox]')`;
  const tr = (n) => `Track ${String(n).padStart(2, '0')}`;
  const all = (await t.ev(order)).split(',');
  const picked = [2, 4, 5, 7, 10].map(tr);
  const hidden = picked.filter((l) => l !== tr(5));

  await t.ev(`${list}.scrollIntoView({ block: 'center' })`);
  await t.ev(`${list}.style.maxHeight = '720px'`);
  for (const l of picked) await t.click(tick(l), 40);
  const gap = await t.ev(`(() => { const [a, b] = ${its}; return b.getBoundingClientRect().top - a.getBoundingClientRect().bottom; })()`);
  const pitch = await t.ev(`(() => { const [a, b] = ${its}; return b.getBoundingClientRect().top - a.getBoundingClientRect().top; })()`);

  await drag(t, grip(tr(5)), 0, pitch * 3.4, {
    hold: 450,
    mid: async () => {
      checkCollapsed(t, 'pointer', await t.ev(probe(sec, 'y')), gap, hidden, tr(5));
      t.expect('pointer: the pile follows the pointer', await t.attr(`${its}.find((e) => e.dataset.anchor)`, 'data-following'), 'true');
    },
  });
  const after = (await t.ev(order)).split(',');
  const { keep } = inOrder(all, picked);
  const at = after.indexOf(picked[0]);
  t.expect('pointer: the block lands as one run, in order', after.slice(at, at + picked.length).join(','), picked.join(','));
  t.expect('pointer: the rest keep their order', after.filter((l) => !picked.includes(l)).join(','), keep.join(','));
  t.ok('pointer: it moved', at !== all.indexOf(picked[0]) || after.join(',') !== all.join(','));
  t.expect('pointer: nothing is left hidden or sliding', await t.ev(`${its}.filter((e) => e.style.transform || e.style.visibility || e.dataset.hidden).length`), 0);

  const before = await t.ev(order);
  const [x, y] = await t.center(grip(tr(5)));
  await t.mouse('mouseMoved', x, y);
  await t.mouse('mousePressed', x, y);
  for (let i = 1; i <= 12; i++) {
    await t.mouse('mouseMoved', x, y + (pitch * 2.2 * i) / 12);
    await t.sleep(16);
  }
  await t.sleep(300);
  t.expect('cancel: the others are hidden while it is held', (await t.ev(probe(sec, 'y'))).hidden, hidden.join(','));
  await t.key('Escape');
  await t.mouse('mouseReleased', x, y + pitch * 2.2);
  await t.sleep(300);
  t.expect('cancel: Escape restores the order', await t.ev(order), before);
  const loose = await t.ev(probe(sec, 'y'));
  t.expect('cancel: and everything is drawn again', loose.loose, 0);

  await t.focus(grip(tr(5)));
  await t.key(' ');
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  await t.sleep(450);
  checkCollapsed(t, 'keyboard', await t.ev(probe(sec, 'y')), gap, hidden, tr(5));
  t.expect('keyboard: the pile is not a pointer follower', await t.attr(`${its}.find((e) => e.dataset.anchor)`, 'data-following'), null);
  await t.key('Escape');
  await t.sleep(300);
  t.expect('keyboard cancel: the order is back', await t.ev(order), before);
  t.expect('keyboard cancel: and everything is drawn again', (await t.ev(probe(sec, 'y'))).loose, 0);

  await t.focus(grip(tr(5)));
  await t.key(' ');
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  await t.key(' ');
  await t.settle('keyboard: the block lands as one run', `(() => { const a = ${order}.split(','); const i = a.indexOf(${JSON.stringify(picked[0])}); return a.slice(i, i + 5).join(','); })()`, picked.join(','));
  const done = (await t.ev(order)).split(',');
  t.expect('keyboard: the rest keep their order', done.filter((l) => !picked.includes(l)).join(','), all.filter((l) => !picked.includes(l)).join(','));
  t.expect('keyboard: nothing is left hidden or sliding', (await t.ev(probe(sec, 'y'))).loose, 0);
};

const sideways = async (t) => {
  const sec = section(/^Sideways/);
  const its = `[...${sec}.querySelectorAll('[data-slot=sortable-item]')]`;
  const order = `${its}.map((e) => e.dataset.sortableLabel).join(',')`;
  const chip = (label) => `${its}.find((e) => e.dataset.sortableLabel === ${JSON.stringify(label)})`;
  const tick = (label) => `${chip(label)}.querySelector('[role=checkbox]')`;
  const words = (label) => `${chip(label)}.querySelector('span:not([data-slot])')`;
  const all = ['rust', 'leptos', 'wasm', 'tailwind', 'trunk', 'serde', 'tokio', 'axum'];
  const picked = ['rust', 'wasm', 'trunk', 'tokio'];
  const hidden = picked.filter((l) => l !== 'trunk');

  await t.ev(`${sec}.scrollIntoView({ block: 'center' })`);
  t.expect('sideways starts in order', await t.ev(order), all.join(','));
  for (const l of picked) await t.click(tick(l), 40);
  const gap = await t.ev(`(() => { const [a, b] = ${its}; return b.getBoundingClientRect().left - a.getBoundingClientRect().right; })()`);

  await drag(t, words('trunk'), 110, 0, {
    hold: 450,
    mid: async () => {
      checkCollapsed(t, 'sideways', await t.ev(probe(sec, 'x')), gap, hidden, 'trunk');
    },
  });
  const after = (await t.ev(order)).split(',');
  const at = after.indexOf('rust');
  t.expect('sideways: the block lands as one run, in order', after.slice(at, at + 4).join(','), picked.join(','));
  t.expect('sideways: the rest keep their order', after.filter((l) => !picked.includes(l)).join(','), all.filter((l) => !picked.includes(l)).join(','));
  t.ok('sideways: it moved', after.join(',') !== all.join(','));
  t.expect('sideways: nothing is left hidden or sliding', (await t.ev(probe(sec, 'x'))).loose, 0);
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
  t.expect('every rendered handle is a tab stop', await t.ev(`${rows}.every((e) => e.querySelector('[role=button]').tabIndex === 0)`), true);
  t.ok('the list is as tall as every row', (await t.ev(`${list}.scrollHeight`)) >= 5000 * 44);

  await t.focus(grip(0));
  await t.key(' ');
  t.ok('picking up is announced from data', /Picked up Track 0001, position 1 of 5000/.test(await t.ev(`${win}.querySelector('[aria-live=assertive]').textContent`)));
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  await t.key('ArrowDown');
  t.ok('a move is announced', /Track 0001 moved to position 4 of 5000/.test(await t.ev(`${win}.querySelector('[aria-live=assertive]').textContent`)));
  t.expect('the progress names the row and its destination', await t.ev(`${win}.querySelector('[aria-live=polite]').textContent`), 'Moving Track 0001. Destination position 4 of 5,000.');
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
  t.expect('the progress is cleared on a cancel', await t.ev(`${win}.querySelector('[aria-live=polite]').textContent`), '');

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
      t.expect('only the held row rides the pointer', await t.ev(`${rows}.filter((e) => e.dataset.following).map((e) => e.dataset.index).join(',')`), '2');
      t.expect('the rest of the block is not in the page', await t.ev(`${rows}.filter((e) => e.dataset.index === '1' || e.dataset.index === '3').length`), 0);
      t.expect('and nothing waits as a ghost', await t.ev(`${win}.querySelectorAll('[data-placeholder]').length`), 0);
      t.expect('the list is two rows shorter, the block being one', await t.ev(`${list}.firstElementChild.style.height`), `${(5000 - 2) * pitch}px`);
      t.expect('one empty slot shows where the block lands', await t.ev(`${win}.querySelectorAll('[data-slot=sortable-window-drop]').length + '/' + ${win}.querySelector('[data-slot=sortable-window-drop]').textContent`), '1/');
      const ys = await t.ev(`${rows}.filter((e) => !e.dataset.following).map((e) => +e.style.transform.match(/translate3d\\(0px, (-?[\\d.]+)px/)[1]).sort((a, b) => a - b)`);
      const jumps = ys.slice(1).map((y, i) => y - ys[i]);
      t.expect('every row is plain and the one gap is the slot', jumps.filter((j) => j !== pitch).join(','), String(pitch * 2));
      const gap = ys.find((y, i) => ys[i + 1] - y === pitch * 2) + pitch;
      t.expect('and the marker is in it', await t.ev(`+${win}.querySelector('[data-slot=sortable-window-drop]').style.transform.match(/translate3d\\(0px, (-?[\\d.]+)px/)[1]`), gap);
    },
  });
  t.expect('a pointer drag lands the block as one run', await t.ev(names(10)), lay(moved, rest, 5).slice(0, 10).join(','));
  t.ok('the move is reported as a block', /Moved 3 items to 5/.test(await t.ev(last)));
  t.expect('the selection is kept', await t.ev(count), '3 selected');
  t.expect('nothing is left riding the pointer', await t.ev(`${rows}.filter((e) => e.dataset.following || e.dataset.count).length`), 0);

  const polite = `${win}.querySelector('[aria-live=polite]').textContent`;
  t.expect('the polite region is empty between moves', await t.ev(polite), '');
  await t.focus(grip(6));
  await t.key(' ');
  t.ok('a block pick-up is announced as one', /Picked up 3 items/.test(await t.ev(live)));
  t.expect('and the progress says what is moving', await t.ev(polite), 'Moving 3 items. Destination position 6 of 5,000.');
  await t.key('Home');
  t.expect('and re-says the destination as it changes', await t.ev(polite), 'Moving 3 items. Destination position 1 of 5,000.');
  t.expect('the block is one stack at the top while it is held', await t.ev(`[${row(6)}.dataset.count, ${row(6)}.style.transform, ${rows}.filter((e) => e.dataset.selected).length].join('|')`), '3|translate3d(0px, 0px, 0px)|1');
  t.expect('and has no marker, being in its slot', await t.ev(`${win}.querySelectorAll('[data-slot=sortable-window-drop]').length`), 0);
  await t.key(' ');
  const second = lay(moved, rest, 0);
  await t.settle('the keyboard moves the block too', names(10), second.slice(0, 10).join(','));
  t.ok('and says where the block went', /Dropped 3 items/.test(await t.ev(live)));
  t.expect('the progress is cleared on the drop', await t.ev(polite), '');
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
  t.expect('a drag takes the far row along too, in list order', after.slice(3, 7), [...moved, far]);
  t.expect('and the rest keep their order', [...after.slice(0, 3), ...after.slice(7)], second.slice(3, 7));
  t.ok('it is reported with the block size', /Moved 4 items to 3/.test(await t.ev(last)));
};

const windowedTen = async (t) => {
  const win = section(/^Windowed/);
  const list = `${win}.querySelector('[data-slot=sortable-window]')`;
  const row = (i) => `${win}.querySelector('[data-index="${i}"]')`;
  const grip = (i) => `${row(i)}.querySelector('[role=button]')`;
  const tick = (i) => `${row(i)}.querySelector('[role=checkbox]')`;
  const rows = `[...${win}.querySelectorAll('[data-slot=sortable-window-item]')]`;
  const last = `document.querySelector('[data-testid=sortable-window-last]').textContent`;
  const count = `document.querySelector('[data-testid=sortable-window-count]').textContent`;
  const polite = `${win}.querySelector('[aria-live=polite]').textContent`;
  const pitch = 44;
  const picked = [2, 5, 9, 14, 30, 200, 900, 2000, 3000, 4999];
  const names = picked.map((i) => `Track ${String(i + 1).padStart(4, '0')}`);
  const at = (i) => `${row(i)}?.textContent.match(/Track \\d+/)?.[0]`;
  const run = async (from, n) => {
    await t.ev(`${list}.scrollTop = ${Math.max(0, from * pitch - 2 * pitch)}`);
    await t.settle(`rows ${from}..${from + n - 1} are in the page`, `${row(from)} !== null && ${row(from + n - 1)} !== null`, true);
    const got = [];
    for (let i = from; i < from + n; i++) got.push(await t.ev(at(i)));
    return got;
  };

  await t.ev(`${list}.scrollIntoView({ block: 'center' })`);
  await t.click(`${win}.querySelector('[role=switch]')`);
  for (const i of picked) {
    await t.ev(`${list}.scrollTop = ${Math.max(0, i * pitch - 2 * pitch)}`);
    await t.settle(`row ${i} is in the page`, `${row(i)} !== null && ${tick(i)} !== null`, true);
    await t.click(tick(i), 80);
  }
  t.expect('ten rows are selected, most of them out of the page', await t.ev(count), '10 selected');
  await t.ev(`${list}.scrollTop = 0`);
  await t.settle('back at the top', `!!${row(0)}`, true);

  const place = (i) => `Math.round(${row(i)}.getBoundingClientRect().top - ${list}.getBoundingClientRect().top)`;
  await t.ev(`${list}.scrollTop = ${2995 * pitch + 10}`);
  await t.settle('deep in the list', `${row(3000)} !== null && ${row(3001)} !== null`, true);
  const spot = await t.ev(`(() => { const b = ${grip(3000)}.getBoundingClientRect(); return [b.x + b.width / 2, b.y + b.height / 2]; })()`);
  const around = async () => Promise.all([2999, 3001, 3002, 3005].map((i) => t.ev(`${row(i)} ? ${place(i)} : null`)));
  const placed = await around();
  await t.mouse('mouseMoved', spot[0], spot[1]);
  await t.mouse('mousePressed', spot[0], spot[1]);
  for (let i = 1; i <= 6; i++) {
    await t.mouse('mouseMoved', spot[0], spot[1] + i);
    await t.sleep(16);
  }
  await t.sleep(300);
  t.expect('deep: eight selected rows above the held one left, and the rows beside it did not move', await around(), placed);
  await t.key('Escape');
  await t.mouse('mouseReleased', spot[0], spot[1] + 6);
  await t.sleep(400);
  t.expect('deep: and a cancel puts them back where they were', await around(), placed);
  await t.ev(`${list}.scrollTop = 0`);
  await t.settle('back at the top', `!!${row(0)}`, true);

  const staying = picked.filter((i) => i !== 5);
  const stack = async (label, expectedHeight) => {
    t.expect(`${label}: the held row counts the block`, await t.attr(`${win}.querySelector('[data-anchor]')`, 'data-count'), '10');
    t.expect(`${label}: of the selection only the held row is in the page`, await t.ev(`${rows}.filter((e) => e.dataset.selected).map((e) => e.dataset.index).join(',')`), '5');
    t.expect(`${label}: none of the other nine is in the page`, await t.ev(`${rows}.filter((e) => ${JSON.stringify(staying)}.includes(+e.dataset.index)).length`), 0);
    t.expect(`${label}: no ghosts`, await t.ev(`${win}.querySelectorAll('[data-placeholder]').length`), 0);
    t.expect(`${label}: the list closed up by nine rows`, await t.ev(`${list}.firstElementChild.style.height`), `${(5000 - 9) * pitch}px`);
    t.ok(`${label}: and the page stays small`, (await t.ev(`${rows}.length`)) < 40);
    t.ok(`${label}: the polite region says what is moving`, /^Moving 10 items\. Destination position [\d,]+ of 5,000\.$/.test(await t.ev(polite)));
  };

  const [x, y] = await t.center(grip(5));
  await t.mouse('mouseMoved', x, y);
  await t.mouse('mousePressed', x, y);
  for (let i = 1; i <= 40; i++) {
    await t.mouse('mouseMoved', x, y + (pitch * 10 + 10) * (i / 40));
    await t.sleep(16);
  }
  await t.sleep(250);
  await stack('pointer');
  t.expect('pointer: only the held row rides', await t.ev(`${rows}.filter((e) => e.dataset.following).map((e) => e.dataset.index).join(',')`), '5');
  t.expect('pointer: exactly one empty drop slot', await t.ev(`${win}.querySelectorAll('[data-slot=sortable-window-drop]').length + '/' + ${win}.querySelector('[data-slot=sortable-window-drop]').textContent`), '1/');
  await t.mouse('mouseReleased', x, y + pitch * 10 + 10);
  await t.sleep(300);
  const to = Number(((await t.ev(last)).match(/Moved 10 items to (\d+)/) || [])[1]);
  t.ok('pointer: the move is reported as ten items', Number.isInteger(to) && to > 5);
  t.expect('pointer: the ten land together at the target, in their old order', await run(to, 10), names);
  t.expect('pointer: nothing is left stacked or marked', await t.ev(`${rows}.filter((e) => e.dataset.count || e.dataset.following).length + ${win}.querySelectorAll('[data-slot=sortable-window-drop]').length`), 0);
  t.expect('pointer: the list is whole again', await t.ev(`${list}.firstElementChild.style.height`), `${5000 * pitch}px`);

  const top = Math.max(0, to * pitch - 2 * pitch);
  const [cx, cy] = await t.center(grip(to + 4));
  const firstRow = `(() => { const top = ${list}.getBoundingClientRect().top; return ${rows}.filter((e) => !e.dataset.following && e.getBoundingClientRect().top >= top - 1).sort((a, b) => a.getBoundingClientRect().top - b.getBoundingClientRect().top)[0].dataset.index; })()`;
  await t.mouse('mouseMoved', cx, cy);
  await t.mouse('mousePressed', cx, cy);
  for (let i = 1; i <= 20; i++) {
    await t.mouse('mouseMoved', cx, cy + 2 * pitch * (i / 20));
    await t.sleep(16);
  }
  await t.sleep(200);
  t.expect('cancel: while held, the other nine are gone', await t.ev(`${rows}.filter((e) => e.dataset.selected).length`), 1);
  await t.sleep(300);
  const shown = await t.ev(firstRow);
  const was = await t.ev(place(shown));
  await t.key('Escape');
  await t.mouse('mouseReleased', cx, cy + 2 * pitch);
  await t.sleep(400);
  const now = await t.ev(place(shown));
  t.ok('cancel: the first whole row in view stays where it was', Math.abs(now - was) <= 1);
  t.expect('cancel: nothing moved', await run(to, 10), names);
  t.expect('cancel: the list is whole again', await t.ev(`${list}.firstElementChild.style.height`), `${5000 * pitch}px`);
  t.expect('cancel: the polite region is empty', await t.ev(polite), '');

  await t.ev(`${list}.scrollTop = ${top}`);
  await t.settle('the block is in the page', `${row(to + 4)} !== null`, true);
  await t.focus(grip(to + 4));
  await t.key(' ');
  for (let i = 0; i < 3; i++) await t.key('ArrowUp');
  t.expect('keyboard: the held row counts the block', await t.attr(`${win}.querySelector('[data-anchor]')`, 'data-count'), '10');
  t.expect('keyboard: only one row of the selection is in the page', await t.ev(`${rows}.filter((e) => e.dataset.selected).length`), 1);
  t.expect('keyboard: no ghosts and no marker', await t.ev(`${win}.querySelectorAll('[data-placeholder], [data-slot=sortable-window-drop]').length`), 0);
  t.expect('keyboard: the list closed up by nine rows', await t.ev(`${list}.firstElementChild.style.height`), `${(5000 - 9) * pitch}px`);
  t.expect('keyboard: the progress says where it would land', await t.ev(polite), `Moving 10 items. Destination position ${(to - 3 + 1).toLocaleString('en-US')} of 5,000.`);
  await t.key(' ');
  await t.settle('keyboard: the block lands three rows up', `${last}`, `Moved 10 items to ${to - 3}`);
  t.expect('keyboard: as ten contiguous rows in their old order', await run(to - 3, 10), names);
  t.expect('keyboard: the list is whole again', await t.ev(`${list}.firstElementChild.style.height`), `${5000 * pitch}px`);

  await t.focus(grip(to - 3 + 4));
  await t.key(' ');
  await t.key('ArrowDown');
  await t.key('Escape');
  t.expect('keyboard cancel: nothing moved', await run(to - 3, 10), names);
  t.expect('keyboard cancel: the list is whole again', await t.ev(`${list}.firstElementChild.style.height`), `${5000 * pitch}px`);
};
