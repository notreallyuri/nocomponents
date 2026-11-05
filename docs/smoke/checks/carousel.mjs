import { demo } from '../cdp.mjs';

export default async (t) => {
  const c = `document.querySelector('main [aria-roledescription=carousel]')`;
  t.expect('region role', await t.attr(c, 'role'), 'region');
  t.ok('has a label', !!(await t.attr(c, 'aria-label')));
  const slides = await t.ev(`[...(${c}).querySelectorAll('[aria-roledescription=slide]')].map((e) => e.getAttribute('aria-label'))`);
  t.ok('slides are labelled', slides.length > 1 && slides.every(Boolean));
  t.ok('slide labels are distinct', new Set(slides).size === slides.length);

  const next = `${demo('[data-slot=carousel-next]')}[0]`;
  const scroller = `(() => { const c = ${c}; return [...c.querySelectorAll('*')].find((e) => e.scrollWidth > e.clientWidth + 1 && getComputedStyle(e).overflowX !== 'visible') })()`;
  t.ok('a scroller exists', await t.exists(scroller));
  const a = await t.ev(`Math.round((${scroller}).scrollLeft)`);
  await t.click(next, 100);
  await t.settle('next scrolls the track', `Math.round((${scroller}).scrollLeft) > ${a}`, true, 3000);
};
