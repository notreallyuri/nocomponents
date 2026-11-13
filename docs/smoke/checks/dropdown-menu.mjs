import { demoBtn, menuItem, q, qa, activeText } from '../cdp.mjs';
import { dragBy } from '../lib.mjs';

const openMenus = `[...document.querySelectorAll('[role=menu]')].filter((m) => m.closest('[data-state]')?.dataset.state !== 'closed' && getComputedStyle(m).visibility !== 'hidden').length`;
const radios = `[...document.querySelectorAll('[role=menuitemradio]')].map((e) => e.textContent.trim() + '=' + e.getAttribute('aria-checked')).join(' ')`;
const viewState = `${demoBtn('View')}.getAttribute('aria-expanded')`;

export default async (t) => {
  // Default menu
  await t.click(demoBtn('Options'));
  await t.settle('options menu opens', openMenus, 1);
  t.expect('label slot', await t.ev(`!!document.querySelector('[data-slot=dropdown-menu-label]')`), true);
  t.expect('separator role', await t.attr(q('[data-slot=dropdown-menu-separator]'), 'role'), 'separator');
  t.expect('destructive item variant', await t.attr(menuItem('Log out'), 'data-variant'), 'destructive');
  t.expect('disabled item', await t.attr(menuItem('Billing'), 'aria-disabled'), 'true');

  // hover focuses an item and skips disabled
  await t.hover(menuItem('Settings'));
  t.expect('hover focuses a plain item', await t.ev(activeText), (await t.text(menuItem('Settings'))));
  await t.hover(menuItem('Billing'));
  t.ok('hover skips a disabled item', (await t.ev(activeText)) !== (await t.text(menuItem('Billing'))));
  await t.key('Escape');
  await t.settle('escape closes', openMenus, 0);
  t.expect('focus returns to trigger after Escape', await t.ev(`document.activeElement === ${demoBtn('Options')}`), true);

  // plain item closes and returns focus
  await t.click(demoBtn('Options'));
  await t.click(menuItem('Settings'));
  await t.settle('plain item closes the menu', openMenus, 0);
  t.expect('a mouse-chosen item leaves focus off the trigger', await t.ev(`document.activeElement === ${demoBtn('Options')}`), false);

  // an item chosen with the keyboard hands focus back
  await t.click(demoBtn('Options'));
  await t.hover(menuItem('Settings'));
  await t.key('Enter');
  await t.settle('Enter on an item closes the menu', openMenus, 0);
  t.expect('focus returns to trigger after Enter on an item', await t.ev(`document.activeElement === ${demoBtn('Options')}`), true);

  // Submenu
  await t.click(demoBtn('More'));
  await t.hover(menuItem('Team'));
  await t.settle('hover opens submenu', `${menuItem('Team')}.getAttribute('aria-expanded')`, 'true');
  await t.click(menuItem('Team'));
  await t.sleep(300);
  t.expect('clicking the sub trigger keeps it open', await t.attr(menuItem('Team'), 'aria-expanded'), 'true');
  t.expect('sub trigger data-state', await t.attr(menuItem('Team'), 'data-state'), 'open');
  t.expect('both menus open', await t.ev(openMenus), 2);
  const sub = await t.ev(`(() => {
    const e = [...document.querySelectorAll('[data-side]')].find((e) => e.textContent.includes('Invite members') && e.hasAttribute('data-align'));
    return e ? [e.dataset.side, !!e.dataset.align] : null;
  })()`);
  t.ok('submenu content carries data-side and data-align', sub && sub[1]);
  await t.key('Escape');
  await t.key('Escape');
  await t.settle('escape closes the tree', openMenus, 0);

  // Choices: checkbox and radio items stay open
  await t.click(demoBtn('View'));
  t.expect('checkbox item role', await t.attr(menuItem('Status bar'), 'role'), 'menuitemcheckbox');
  t.expect('radio group count', await t.ev(`document.querySelectorAll('[role=menuitemradio]').length > 1`), true);
  const checkedBefore = await t.attr(menuItem('Status bar'), 'aria-checked');

  await t.click(menuItem('Status bar'));
  t.expect('click on a checkbox item keeps the menu open', await t.attr(demoBtn('View'), 'aria-expanded'), 'true');
  t.ok('checkbox item toggled', (await t.attr(menuItem('Status bar'), 'aria-checked')) !== checkedBefore);
  t.expect('data-state follows aria-checked', await t.ev(`${menuItem('Status bar')}.dataset.state`), (await t.attr(menuItem('Status bar'), 'aria-checked')) === 'true' ? 'checked' : 'unchecked');
  await t.click(menuItem('Status bar'));
  t.expect('toggles back', await t.attr(menuItem('Status bar'), 'aria-checked'), checkedBefore);

  await t.click(menuItem('Top'));
  t.expect('click on a radio keeps the menu open', await t.attr(demoBtn('View'), 'aria-expanded'), 'true');
  t.ok('radio picked', (await t.ev(radios)).includes('Top=true'));
  t.ok('only one radio checked', (await t.ev(radios)).split(' ').filter((s) => s.endsWith('=true')).length === 1);

  // keyboard: arrow order skips labels and separators, Enter/Space toggle and stay open
  const order = [];
  for (let i = 0; i < 8; i++) {
    await t.key('ArrowDown');
    order.push(await t.ev(activeText));
  }
  t.ok('arrow order never lands on a label or separator', await t.ev(`(() => {
    const e = document.activeElement;
    return e.matches('[role^=menuitem]');
  })()`));
  t.ok('arrow walk visits several distinct items', new Set(order).size >= 4);

  await t.hover(menuItem('Right'));
  await t.key('Enter');
  t.expect('Enter on a radio keeps the menu open', await t.attr(demoBtn('View'), 'aria-expanded'), 'true');
  t.ok('Enter picks the radio', (await t.ev(radios)).includes('Right=true'));

  const y0 = await t.ev('scrollY');
  await t.hover(menuItem('Top'));
  await t.key(' ');
  t.expect('Space keeps the menu open', await t.attr(demoBtn('View'), 'aria-expanded'), 'true');
  t.ok('Space picks the radio', (await t.ev(radios)).includes('Top=true'));
  t.expect('Space does not scroll the page', (await t.ev('scrollY')) === y0, true);

  await t.hover(menuItem('Status bar'));
  const sb = await t.attr(menuItem('Status bar'), 'aria-checked');
  await t.key(' ');
  t.ok('Space toggles a checkbox item', (await t.attr(menuItem('Status bar'), 'aria-checked')) !== sb);
  t.expect('still open', await t.attr(demoBtn('View'), 'aria-expanded'), 'true');

  await t.click(menuItem('Reset layout'));
  await t.settle('a plain item closes the menu', viewState, 'false');
  t.expect('a mouse click does not return focus to View', await t.ev(`document.activeElement === ${demoBtn('View')}`), false);

  // On phones: the same menu is a bottom sheet, and only below md.
  const sheetEl = `document.querySelector('[data-slot=dropdown-menu-content]')`;
  const frameEl = `document.querySelector('[data-slot=menu-sheet]')`;
  const presentation = `${sheetEl}?.dataset.presentation ?? null`;
  const open = `${demoBtn('Sort by')}.getAttribute('aria-expanded')`;

  await t.viewport(1280, 900);
  await t.click(demoBtn('Sort by'), 400);
  t.expect('floating at 1280', await t.ev(presentation), 'floating');
  t.expect('no scrim at 1280', await t.ev(`!!document.querySelector('[data-slot=menu-sheet-scrim]')`), false);
  await t.key('Escape', 400);

  await t.viewport(390, 800, true);
  await t.sleep(300);
  await t.click(demoBtn('Sort by'), 500);
  t.expect('sheet at 390', await t.ev(presentation), 'sheet');
  t.ok('scrim is present', await t.ev(`!!document.querySelector('[data-slot=menu-sheet-scrim]')`));
  t.expect('title row shows sheet_title', await t.ev(`document.querySelector('[data-slot=menu-sheet-title]')?.textContent`), 'Sort by');
  t.ok('grabber is present', await t.ev(`!!document.querySelector('[data-slot=sheet-grabber]')`));
  const box = await t.ev(`(() => { const r = ${frameEl}.getBoundingClientRect(); return [r.left, r.right, r.bottom, innerWidth, innerHeight]; })()`);
  t.ok('anchored to the bottom edge and full width', box && Math.abs(box[2] - box[4]) <= 1 && box[0] <= 1 && Math.abs(box[1] - box[3]) <= 1);
  t.ok('menu has focus inside', await t.ev(`${sheetEl}.contains(document.activeElement)`));
  t.ok('the header is a sibling of the menu, not inside it', await t.ev(`!${sheetEl}.querySelector('[data-slot=menu-sheet-header]') && ${frameEl}.querySelector('[data-slot=menu-sheet-header]')?.nextElementSibling === ${sheetEl}`));

  await t.click(menuItem('Size'), 400);
  await t.settle('an item click closes the sheet', open, 'false');

  await t.click(demoBtn('Sort by'), 500);
  t.expect('reopens as a sheet', await t.ev(presentation), 'sheet');
  await t.key('ArrowDown');
  t.ok('arrows move through the items in a sheet', await t.ev(`document.activeElement.matches('[role^=menuitem]')`));
  await t.key('Escape', 400);
  await t.settle('Escape closes the sheet', open, 'false');
  t.expect('focus returns to the trigger after Escape', await t.ev(`document.activeElement === ${demoBtn('Sort by')}`), true);

  const header = `document.querySelector('[data-slot=menu-sheet-header]')`;
  const closeBtn = `document.querySelector('[data-slot=menu-sheet-close]')`;

  await t.click(demoBtn('Sort by'), 500);
  t.ok('the sheet header has a close button', await t.exists(closeBtn));
  t.expect('named Close', await t.attr(closeBtn, 'aria-label'), 'Close');
  t.ok('drawn with the cross glyph', await t.ev(`!!${closeBtn}.querySelector('svg')`));
  await t.click(closeBtn, 400);
  await t.settle('the close button closes the sheet', open, 'false');

  await t.click(demoBtn('Sort by'), 500);
  t.expect('at rest a sheet is not dragging', await t.ev(`${frameEl}.dataset.dragging ?? null`), null);
  let midDrag;
  await dragBy(t, header, 30, {
    mid: async () => {
      midDrag = await t.ev(`[${frameEl}.dataset.dragging, ${frameEl}.style.transform, ${frameEl}.style.transition]`);
    },
  });
  t.expect('a drag marks the panel as dragging', midDrag?.[0], 'true');
  t.ok('moves it with the pointer', /30px/.test(midDrag?.[1] ?? ''), midDrag?.[1]);
  t.ok('with no easing', /none/.test(midDrag?.[2] ?? ''), midDrag?.[2]);
  t.expect('a short drag springs back and stays open', await t.attr(demoBtn('Sort by'), 'aria-expanded'), 'true');
  t.expect('the mark is gone after the release', await t.ev(`${frameEl}.dataset.dragging ?? null`), null);
  t.ok('and the panel is back at rest', await t.ev(`${frameEl}.style.transform === ''`));

  await dragBy(t, header, 400);
  await t.settle('a drag of the header past the threshold dismisses the sheet', open, 'false');

  await t.click(demoBtn('Sort by'), 500);
  t.expect('it reopens at rest', await t.ev(`${frameEl}.style.transform === ''`), true);
  await t.key('Escape', 400);
  await t.settle('Escape closes it again', open, 'false');

  await t.click(demoBtn('Sort by'), 500);
  await t.clickAt(195, 60, 400);
  await t.settle('a tap on the scrim closes the sheet', open, 'false');

  await t.viewport(1280, 900);
};
