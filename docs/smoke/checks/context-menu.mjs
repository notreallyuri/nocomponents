import { demo } from '../cdp.mjs';

const trigger = `${demo('[data-slot=context-menu-trigger]')}[0]`;
const content = `document.querySelector('[data-slot=context-menu-content]')`;
const isOpen = `(() => { const c = ${content}; return !!c && c.dataset.state === 'open' && getComputedStyle(c).visibility !== 'hidden'; })()`;
const origin = `(() => { const r = ${content}.getBoundingClientRect(); return [r.left, r.top]; })()`;

const touch = (t, type, points) => t.send('Input.dispatchTouchEvent', { type, touchPoints: points });

export default async (t) => {
  await t.click(trigger, 200);
  t.expect('a left click does not open it', await t.ev(isOpen), false);

  await t.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
  try {
    const [cx, cy] = await t.center(trigger);
    const x = Math.round(cx);
    const y = Math.round(cy);

    await touch(t, 'touchStart', [{ x, y }]);
    await t.sleep(250);
    t.expect('a press shorter than the delay has not opened it', await t.ev(isOpen), false);
    await t.sleep(450);
    t.expect('a held touch opens the menu', await t.ev(isOpen), true);
    const [left, top] = await t.ev(origin);
    t.ok('it opens at the touch point', Math.abs(left - x) < 40 && Math.abs(top - y) < 40, `[${left}, ${top}] vs [${x}, ${y}]`);
    await touch(t, 'touchEnd', []);
    await t.sleep(1200);
    t.expect('it stays open through release and a native contextmenu', await t.ev(isOpen), true);
    t.expect('and has not moved', await t.ev(origin), [left, top]);
    await t.key('Escape', 400);
    t.expect('Escape closes it', await t.ev(isOpen), false);

    await touch(t, 'touchStart', [{ x, y }]);
    for (let i = 1; i <= 4; i++) {
      await touch(t, 'touchMove', [{ x: x + i * 8, y }]);
      await t.sleep(30);
    }
    await t.sleep(700);
    t.expect('a press that moves is not a long press', await t.ev(isOpen), false);
    await touch(t, 'touchEnd', []);

    await touch(t, 'touchStart', [{ x, y }]);
    await t.sleep(150);
    await touch(t, 'touchEnd', []);
    await t.sleep(700);
    t.expect('a press released early is not a long press', await t.ev(isOpen), false);

    t.ok('the trigger turns the iOS callout off', (await t.attr(trigger, 'style'))?.includes('-webkit-touch-callout'));
  } finally {
    await t.send('Emulation.setTouchEmulationEnabled', { enabled: false });
  }
};
