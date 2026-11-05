export default async (t) => {
  t.ok('columns render', (await t.ev(`document.querySelectorAll('main [data-slot=kanban-column]').length`)) >= 2);
  t.ok('cards render', (await t.ev(`document.querySelectorAll('main [data-slot=kanban-card]').length`)) >= 3);
};
