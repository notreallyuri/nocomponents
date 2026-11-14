import { demo, menuItem } from '../cdp.mjs';
import { assertPageFrozen, lockState, touchTap, withTouch } from '../lib.mjs';

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
    const [left2, top2] = await t.ev(origin);
    t.ok('and has not moved', Math.abs(left2 - left) <= 1 && Math.abs(top2 - top) <= 1, `[${left2}, ${top2}] vs [${left}, ${top}]`);
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

  await submenus(t);
};

const submenuTrigger = `${demo('[data-slot=context-menu-trigger]')}[1]`;
const phoneTrigger = `${demo('[data-slot=context-menu-trigger]')}[2]`;
const subTrigger = `[...document.querySelectorAll('[data-slot=dropdown-menu-sub-trigger]')].find((e) => e.textContent.trim().startsWith('Move to'))`;
const subContent = `document.querySelector('[data-slot=dropdown-menu-sub-content]')`;
const visible = (item) => `(() => { const r = (${item}).getBoundingClientRect(); return r.height > 0 && r.top >= 0 && r.bottom <= innerHeight; })()`;

async function submenus(t) {
  await t.viewport(1280, 900);
  const [bx, by] = await t.center(submenuTrigger);
  await t.mouse('mouseMoved', bx, by);
  await t.mouse('mousePressed', bx, by, 'right');
  await t.mouse('mouseReleased', bx, by, 'right');
  await t.sleep(300);
  t.expect('desktop: a floating menu does not lock the page', await t.ev(lockState), [false, '', '']);
  await t.hover(menuItem('Move to'), 400);
  t.expect('desktop: hovering the sub trigger opens the floating submenu', await t.attr(menuItem('Move to'), 'aria-expanded'), 'true');
  t.expect('desktop: the submenu is a floating menu', await t.ev(`${subContent}.getAttribute('role')`), 'menu');
  t.expect('desktop: portalled out of the parent menu', await t.ev(`${subContent}.closest('[data-slot=context-menu-content]')`), null);
  await t.key('Escape', 300);
  await t.key('Escape', 300);

  await t.viewport(390, 800, true);
  await t.sleep(300);
  await withTouch(t, async () => {
    const open = async () => {
      const [cx, cy] = await t.center(phoneTrigger);
      await t.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: Math.round(cx), y: Math.round(cy) }] });
      await t.sleep(700);
      await t.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
      await t.sleep(700);
    };
    const tap = async (item) => {
      const [x, y] = await t.center(item);
      await touchTap(t, Math.round(x), Math.round(y));
    };

    await open();
    const menu = `document.querySelector('[data-slot=context-menu-content]')`;
    t.expect('phone: the context menu is a sheet', await t.ev(`${menu}?.dataset.presentation`), 'sheet');
    await assertPageFrozen(t, [195, 60], 'context menu sheet');
    t.expect('phone: the sub trigger is collapsed', await t.attr(subTrigger, 'aria-expanded'), 'false');
    t.expect('phone: and says it is inline', await t.attr(subTrigger, 'data-presentation'), 'sheet');

    await tap(subTrigger);
    t.expect('a tap expands the submenu', await t.attr(subTrigger, 'aria-expanded'), 'true');
    await t.sleep(500);
    t.expect('and it stays expanded after the finger lifts', await t.attr(subTrigger, 'aria-expanded'), 'true');
    t.expect('the submenu sits inside the sheet, not a portal', await t.ev(`!!${menu}.contains(${subContent})`), true);
    t.expect('directly below its trigger', await t.ev(`${subTrigger}.parentElement === ${subContent}.parentElement && ${subTrigger}.nextElementSibling === ${subContent}`), true);
    t.ok('its items can be reached', await t.ev(visible(menuItem('Forward'))));
    t.ok('the chevron has turned', await t.ev(`getComputedStyle(${subTrigger}.querySelector('[data-slot=dropdown-menu-sub-icon]')).rotate === '90deg'`));

    await tap(subTrigger);
    t.expect('a second tap folds it', await t.attr(subTrigger, 'aria-expanded'), 'false');
    t.expect('and removes its items', await t.ev(`!!${subContent}`), false);

    await t.focus(subTrigger);
    await t.key('ArrowRight', 300);
    t.expect('ArrowRight expands it', await t.attr(subTrigger, 'aria-expanded'), 'true');
    t.ok('and focuses its first item', (await t.ev(`document.activeElement.textContent.trim()`)) === 'Front');
    await t.key('ArrowDown');
    t.expect('ArrowDown steps through the inline items', await t.ev(`document.activeElement.textContent.trim()`), 'Forward');
    await t.key('ArrowLeft');
    t.expect('ArrowLeft folds it', await t.attr(subTrigger, 'aria-expanded'), 'false');
    t.expect('and returns to the trigger', await t.ev(`document.activeElement === ${subTrigger}`), true);

    await tap(subTrigger);
    await tap(menuItem('Backward'));
    await t.settle('choosing an inline item closes the whole sheet', `${menu} ? 'open' : 'gone'`, 'gone');
    t.expect('the page is unlocked again', await t.ev(lockState), [false, '', '']);
  });
  await t.viewport(1280, 900);
}
