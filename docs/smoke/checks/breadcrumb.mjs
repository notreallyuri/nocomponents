export default async (t) => {
  t.expect('nav is labelled', await t.attr(`document.querySelector('main nav[aria-label]')`, 'aria-label'), 'Breadcrumb');
  t.expect('no link has an empty href', await t.ev(`document.querySelectorAll('main a[href=""]').length`), 0);
  t.expect('the current page is marked', await t.ev(`!!document.querySelector('main [data-slot=breadcrumb-page][aria-current=page]')`), true);
};
