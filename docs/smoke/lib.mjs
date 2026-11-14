// Pieces several checks share.
import { demo } from './cdp.mjs';

// The first demo trigger that opens a dialog-like layer, excluding the header's theme switcher
// (which is also aria-haspopup=dialog).
export const dialogTrigger = (i = 0) =>
  `${demo('[aria-haspopup=dialog]')}.filter((e) => e.dataset.slot !== 'popover-trigger')[${i}]`;

// A trigger/layer round trip: opens, wires aria-expanded/aria-controls to a real id, closes on
// Escape and gives focus back.
export async function layerRoundTrip(t, trigger, { role = 'dialog', settleMs = 600 } = {}) {
  t.expect('trigger starts collapsed', await t.attr(trigger, 'aria-expanded'), 'false');
  await t.click(trigger, settleMs);
  t.expect('aria-expanded after open', await t.attr(trigger, 'aria-expanded'), 'true');
  t.expect('aria-controls names an existing element', await t.ev(`!!document.getElementById((${trigger}).getAttribute('aria-controls'))`), true);
  if (role) t.ok(`a [role=${role}] is present`, await t.exists(`document.querySelector('[role=${role}]')`));
  await t.key('Escape', settleMs);
  t.expect('Escape collapses the trigger', await t.attr(trigger, 'aria-expanded'), 'false');
}

// Presses on `from` (a selector expression), drags it by `dy` pixels down (negative is up) in
// steps, optionally pauses, runs `mid` while the pointer is still down, and releases. `hold` is how
// long the pointer rests before the release: past the library's flick window it is a placement,
// not a throw.
export async function dragBy(t, from, dy, { hold = 250, mid } = {}) {
  const [x, y] = await t.center(from);
  await t.mouse('mouseMoved', x, y);
  await t.mouse('mousePressed', x, y);
  const steps = Math.max(8, Math.ceil(Math.abs(dy) / 12));
  for (let i = 1; i <= steps; i++) {
    await t.mouse('mouseMoved', x, y + (dy * i) / steps);
    await t.sleep(16);
  }
  if (hold) await t.sleep(hold);
  if (mid) await mid();
  await t.mouse('mouseReleased', x, y + dy);
  await t.sleep(500);
}

// What a scroll-locked page looks like: the attribute on <html>, and the inline overflow the lock
// writes on it and on <body>. Unlocked it is `[false, '', '']`.
export const lockState = `[document.documentElement.hasAttribute('data-scroll-locked'), document.documentElement.style.overflow, document.body.style.overflow]`;

// Dispatches one touch gesture: down at (x, y), moves to (x, y + dy) in steps, up. Needs touch
// emulation on, which `withTouch` does.
export async function touchDrag(t, x, y, dy, { steps = 6, hold = 0 } = {}) {
  const send = (type, points) => t.send('Input.dispatchTouchEvent', { type, touchPoints: points });
  await send('touchStart', [{ x, y }]);
  for (let i = 1; i <= steps; i++) {
    await send('touchMove', [{ x, y: y + (dy * i) / steps }]);
    await t.sleep(16);
  }
  if (hold) await t.sleep(hold);
  await send('touchEnd', []);
  await t.sleep(300);
}

export async function touchTap(t, x, y, wait = 400) {
  await t.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y }] });
  await t.sleep(60);
  await t.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  await t.sleep(wait);
}

export async function withTouch(t, fn) {
  await t.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
  try {
    await fn();
  } finally {
    await t.send('Emulation.setTouchEmulationEnabled', { enabled: false });
  }
}

// While a sheet is open the page behind it must not move: not by the wheel over the scrim, and not
// by a finger dragged on its header. `scrimPoint` is a spot on the scrim. The sheet is left open.
export async function assertPageFrozen(t, scrimPoint, label) {
  t.expect(`${label}: an open sheet locks the page`, await t.ev(lockState), [true, 'hidden', 'hidden']);
  const y0 = await t.ev('scrollY');
  await t.send('Input.dispatchMouseEvent', { type: 'mouseWheel', x: scrimPoint[0], y: scrimPoint[1], deltaX: 0, deltaY: 400 });
  await t.sleep(250);
  t.expect(`${label}: a wheel over the scrim does not scroll the page`, await t.ev('scrollY'), y0);
  await withTouch(t, async () => {
    const [hx, hy] = await t.center(`document.querySelector('[data-slot=menu-sheet-header]')`);
    await touchDrag(t, Math.round(hx), Math.round(hy), -40, { hold: 300 });
    t.expect(`${label}: a finger dragged on the header does not scroll the page`, await t.ev('scrollY'), y0);
    t.expect(`${label}: and the sheet is still open`, await t.ev(`!!document.querySelector('[data-slot=menu-sheet-header]')`), true);
  });
}

// A finger dragged over the scrim must not scroll the page. A press there is an outside press, so
// this closes the sheet as well.
export async function assertScrimDragFrozen(t, scrimPoint, label) {
  const y0 = await t.ev('scrollY');
  await withTouch(t, async () => {
    await touchDrag(t, scrimPoint[0], scrimPoint[1] + 60, -50);
  });
  t.expect(`${label}: a finger dragged over the scrim does not scroll the page`, await t.ev('scrollY'), y0);
}
