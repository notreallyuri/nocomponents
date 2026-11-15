import { touchTap, withTouch } from '../lib.mjs';

export default async (t) => {
  const pressed = `[...document.querySelectorAll('main button[aria-pressed]')]`;
  t.ok('every toggle button has an accessible name', await t.ev(`${pressed}.every((b) => b.getAttribute('aria-label') || b.textContent.trim())`));

  const mute = `document.querySelector('main button[aria-label=Mute]')`;
  t.ok('a Mute button exists', await t.exists(mute));
  t.expect('starts unpressed', await t.attr(mute, 'aria-pressed'), 'false');
  await t.click(mute, 300);
  t.expect('pressing sets aria-pressed', await t.attr(mute, 'aria-pressed'), 'true');
  t.expect('the label stays fixed', await t.attr(mute, 'aria-label'), 'Mute');
  t.expect('the video is muted', await t.ev(`(${mute}).closest('[data-slot=video-player]')?.querySelector('video')?.muted`), true);

  // On touch the surface shows and hides the controls and leaves playback to their button; the
  // controls also put themselves away once the video plays, with no pointer involved.
  const player = `document.querySelector('main [data-slot=video-player]')`;
  const video = `(${player}).querySelector('video')`;
  const controls = `(${player}).querySelector('[data-slot=video-player-controls]')`;
  const shown = `(${controls}).getAttribute('data-active') === 'true'`;
  await t.ev(`(${player}).scrollIntoView({ block: 'center' })`);
  await t.ev(`(() => { const v = ${video}; v.muted = true; return v.play().then(() => true, () => false); })()`);
  await t.settle('the video plays', `!(${video}).paused`, true);
  await t.settle('the controls put themselves away once it plays', shown, false);
  const [x, y] = await t.ev(`(() => { const r = (${player}).getBoundingClientRect(); return [Math.round(r.x + r.width / 2), Math.round(r.y + r.height / 3)]; })()`);
  await withTouch(t, async () => {
    await touchTap(t, x, y, 300);
    t.ok('a touch tap shows the controls', await t.ev(shown));
    t.ok('and does not pause', await t.ev(`!(${video}).paused`));
    await touchTap(t, x, y, 300);
    t.ok('a second touch tap hides them', !(await t.ev(shown)));
    t.ok('and still does not pause', await t.ev(`!(${video}).paused`));
  });
};
