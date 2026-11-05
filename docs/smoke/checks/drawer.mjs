import { dialogTrigger, layerRoundTrip } from '../lib.mjs';
import { demo } from '../cdp.mjs';

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
};
