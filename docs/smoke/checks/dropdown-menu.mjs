import { demoBtn, menuItem, q, qa, activeText } from '../cdp.mjs';

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
  t.expect('focus returns to trigger after a plain item', await t.ev(`document.activeElement === ${demoBtn('Options')}`), true);

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
  t.expect('focus returns to View', await t.ev(`document.activeElement === ${demoBtn('View')}`), true);
};
