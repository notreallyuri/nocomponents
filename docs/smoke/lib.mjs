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
