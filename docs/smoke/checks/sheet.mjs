import { dialogTrigger, layerRoundTrip } from '../lib.mjs';

export default async (t) => {
  const trig = dialogTrigger();
  t.ok('a sheet trigger exists', await t.exists(trig));
  t.expect('trigger slot', await t.attr(trig, 'data-slot'), 'sheet-trigger');
  await layerRoundTrip(t, trig);
};
