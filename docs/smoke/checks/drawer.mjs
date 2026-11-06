import { dialogTrigger, layerRoundTrip } from '../lib.mjs';
import { demo, demoBtn } from '../cdp.mjs';

export default async (t) => {
  const trig = dialogTrigger();
  t.ok('a drawer trigger exists', await t.exists(trig));
  t.expect('trigger slot', await t.attr(trig, 'data-slot'), 'drawer-trigger');
  await layerRoundTrip(t, trig, { settleMs: 700 });

  const nonModal = `${demo('[aria-haspopup=dialog]')}.find((e) => e.dataset.slot !== 'popover-trigger' && /non.?modal/i.test(e.textContent))`;
  if (!(await t.exists(nonModal))) return t.fail('non-modal demo', 'no trigger mentioning non-modal');
  await t.click(nonModal, 700);
  t.expect('non-modal opens', await t.attr(nonModal, 'aria-expanded'), 'true');
  await t.click(nonModal, 800);
  t.expect('a second trigger click does not reopen it', await t.attr(nonModal, 'aria-expanded'), 'false');

  // Snap points
  const panel = `document.querySelector('[data-slot=drawer]')`;
  const handle = `document.querySelector('[data-slot=drawer-handle]')`;
  const snap = `${panel}?.dataset.snap ?? null`;
  const closed = `(!${panel} || ${panel}.dataset.state === 'closed')`;
  const snapTrigger = demoBtn('Open asset info');
  const viewportHeight = await t.ev('innerHeight');

  const dragHandle = async (dy) => {
    await t.waitFor(handle);
    const r = await t.ev(`(() => { const b = ${handle}.getBoundingClientRect(); return [b.x + b.width / 2, b.y + b.height / 2]; })()`);
    const [x, y] = r;
    await t.mouse('mouseMoved', x, y);
    await t.mouse('mousePressed', x, y);
    const steps = 20;
    for (let i = 1; i <= steps; i++) {
      await t.mouse('mouseMoved', x, y + (dy * i) / steps);
      await t.sleep(20);
    }
    await t.sleep(250);
    await t.mouse('mouseReleased', x, y + dy);
    await t.sleep(700);
  };

  await t.click(snapTrigger, 900);
  t.expect('the snapping drawer opens at its first point', await t.ev(snap), '0');
  const peek = await t.ev(`innerHeight - ${panel}.getBoundingClientRect().top`);
  t.ok('which is 40 percent of the viewport', Math.abs(peek - 0.4 * viewportHeight) < 0.04 * viewportHeight, `${peek} of ${viewportHeight}`);

  await dragHandle(-0.1 * viewportHeight);
  t.expect('a short drag up settles back on the first point', await t.ev(snap), '0');

  await dragHandle(-0.4 * viewportHeight);
  t.expect('a drag up past the midpoint settles on the second', await t.ev(snap), '1');
  const full = await t.ev(`innerHeight - ${panel}.getBoundingClientRect().top`);
  t.ok('which is 88 percent of the viewport', Math.abs(full - 0.88 * viewportHeight) < 0.04 * viewportHeight, `${full} of ${viewportHeight}`);

  await dragHandle(0.7 * viewportHeight);
  await t.settle('a drag down below the floor dismisses it', closed, true);

  await t.click(snapTrigger, 900);
  t.expect('and it reopens at the first point', await t.ev(snap), '0');
  await t.key('Escape', 700);
};
