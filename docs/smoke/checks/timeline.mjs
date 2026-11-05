const timelines = `[...document.querySelectorAll('main [data-slot=timeline]')].map((t) => t.dataset.orientation + ':' + [...t.querySelectorAll('[data-slot=timeline-item]')].map((e) => (e.dataset.side ?? '?')[0] + (e.hasAttribute('data-last') ? 'L' : '')).join(''))`;

export default async (t) => {
  const all = await t.ev(timelines);
  t.ok('several timelines render', all.length >= 4);
  t.ok('every timeline marks exactly its last item', all.every((s) => (s.match(/L/g) ?? []).length === 1 && s.endsWith('L')));
  t.ok('an alternating timeline alternates sides', all.some((s) => /^vertical:(es)+L$|^vertical:(se)+L$/.test(s)));
  t.ok('a one-sided timeline stays on one side', all.some((s) => /^vertical:(e+|s+)L$/.test(s)));
};
