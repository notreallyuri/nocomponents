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
};
