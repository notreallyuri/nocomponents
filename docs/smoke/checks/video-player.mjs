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
};
