use crate::{
    cn,
    components::{
        checkbox::Checkbox,
        table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
    },
    icons::chevron::{ChevronDown, ChevronUp},
    primitives::table::{SelectionState, SortDirection, TableSelection, TableSort},
};
use leptos::prelude::*;
use std::cmp::Ordering;

pub struct DataTableColumn<T: Send + Sync + 'static> {
    pub key: &'static str,
    pub header: &'static str,
    pub cell: Callback<T, AnyView>,
    pub compare: Option<Callback<(T, T), Ordering>>,
    pub class: &'static str,
}

impl<T: Send + Sync + 'static> DataTableColumn<T> {
    pub fn new(key: &'static str, header: &'static str, cell: Callback<T, AnyView>) -> Self {
        Self {
            key,
            header,
            cell,
            compare: None,
            class: "",
        }
    }

    pub fn sortable(mut self, compare: Callback<(T, T), Ordering>) -> Self {
        self.compare = Some(compare);
        self
    }

    pub fn class(mut self, class: &'static str) -> Self {
        self.class = class;
        self
    }
}

#[component]
pub fn DataTable<T>(
    #[prop(into)] rows: Signal<Vec<T>>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = None, into)] row_id: Option<Callback<T, String>>,
    #[prop(default = None, into)] selection: Option<TableSelection>,
    #[prop(default = None, into)] sort: Option<TableSort>,
    #[prop(default = "No results.", into)] empty: &'static str,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let sort = sort.unwrap_or_default();
    let columns = StoredValue::new(columns);

    let sorted = Memo::new(move |_| {
        let mut rows = rows.get();
        let Some(column) = sort.column.get() else {
            return rows;
        };

        let compare = columns.with_value(|columns| {
            columns
                .iter()
                .find(|candidate| candidate.key == column)
                .and_then(|column| column.compare)
        });

        if let Some(compare) = compare {
            let descending = sort.direction.get() == SortDirection::Descending;
            rows.sort_by(|a, b| {
                let order = compare.run((a.clone(), b.clone()));
                if descending { order.reverse() } else { order }
            });
        }

        rows
    });

    let page_ids = Memo::new(move |_| match row_id {
        Some(row_id) => sorted
            .get()
            .into_iter()
            .map(|row| row_id.run(row))
            .collect::<Vec<_>>(),
        None => Vec::new(),
    });

    let selecting = row_id.is_some() && selection.is_some();

    let cells = StoredValue::new(columns.with_value(|columns| {
        columns
            .iter()
            .map(|column| (column.class, column.cell))
            .collect::<Vec<_>>()
    }));

    view! {
        <Table class=class>
            <TableHeader>
                <TableRow>
                    {move || {
                        selecting
                            .then(|| {
                                let selection = selection.expect("checked by `selecting`");
                                let all = RwSignal::new(false);

                                Effect::new(move |_| {
                                    let state = selection.state(&page_ids.get());
                                    let checked = state == SelectionState::All;
                                    if all.get_untracked() != checked {
                                        all.set(checked);
                                    }
                                });

                                view! {
                                    <TableHead class="w-10">
                                        <span
                                            data-state=move || {
                                                match selection.state(&page_ids.get()) {
                                                    SelectionState::None => "none",
                                                    SelectionState::Some => "some",
                                                    SelectionState::All => "all",
                                                }
                                            }
                                            class="inline-flex data-[state=some]:opacity-60"
                                        >
                                            <Checkbox
                                                checked=all
                                                attr:aria-label="Select all rows"
                                                on:click=move |_| {
                                                    let ids = page_ids.get_untracked();
                                                    let picked = selection.state(&ids)
                                                        == SelectionState::All;
                                                    selection.set_all(&ids, !picked);
                                                }
                                            />
                                        </span>
                                    </TableHead>
                                }
                            })
                    }}
                    {columns
                        .with_value(|columns| {
                            columns
                                .iter()
                                .map(|column| {
                                    let key = column.key;
                                    let header = column.header;
                                    let class = column.class;

                                    match column.compare {
                                        None => {
                                            view! { <TableHead class=class>{header}</TableHead> }
                                                .into_any()
                                        }
                                        Some(_) => {
                                            view! {
                                                <TableHead
                                                    class=class
                                                    attr:aria-sort=move || sort.aria_sort(key)
                                                >
                                                    <button
                                                        type="button"
                                                        on:click=move |_| sort.toggle(key)
                                                        class="group/sort -ml-2 inline-flex h-7 items-center gap-1 rounded-md px-2 text-left transition-colors hover:bg-muted focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none"
                                                    >
                                                        {header}
                                                        <span class="text-muted-foreground opacity-0 transition-opacity group-hover/sort:opacity-60 data-[active]:opacity-100" data-active=move || sort.is_sorted_by(key).then_some("")>
                                                            {move || {
                                                                if sort.is_sorted_by(key)
                                                                    && sort.direction.get() == SortDirection::Descending
                                                                {
                                                                    view! { <ChevronDown class="size-3.5" /> }.into_any()
                                                                } else {
                                                                    view! { <ChevronUp class="size-3.5" /> }.into_any()
                                                                }
                                                            }}
                                                        </span>
                                                    </button>
                                                </TableHead>
                                            }
                                                .into_any()
                                        }
                                    }
                                })
                                .collect_view()
                        })}
                </TableRow>
            </TableHeader>

            <TableBody>
                {move || {
                    let rows = sorted.get();
                    let width = columns.with_value(|columns| columns.len())
                        + usize::from(selecting);

                    if rows.is_empty() {
                        return view! {
                            <TableRow>
                                <TableCell
                                    attr:colspan=width.to_string()
                                    class="h-24 text-center text-muted-foreground"
                                >
                                    {empty}
                                </TableCell>
                            </TableRow>
                        }
                            .into_any();
                    }

                    rows.into_iter()
                        .map(|row| {
                            let id = row_id.map(|row_id| row_id.run(row.clone()));
                            let picked = {
                                let id = id.clone();
                                Signal::derive(move || {
                                    match (&id, selection) {
                                        (Some(id), Some(selection)) => selection.is_selected(id),
                                        _ => false,
                                    }
                                })
                            };

                            view! {
                                <TableRow attr:data-selected=move || picked.get().then_some("")>
                                    {selecting
                                        .then(|| {
                                            let id = id.clone().expect("checked by `selecting`");
                                            let selection = selection.expect("checked by `selecting`");
                                            let checked = RwSignal::new(picked.get_untracked());

                                            Effect::new(move |_| {
                                                let picked = picked.get();
                                                if checked.get_untracked() != picked {
                                                    checked.set(picked);
                                                }
                                            });

                                            view! {
                                                <TableCell class="w-10">
                                                    <Checkbox
                                                        checked=checked
                                                        attr:aria-label="Select row"
                                                        on:click=move |_| selection.toggle(&id)
                                                    />
                                                </TableCell>
                                            }
                                        })}
                                    {cells
                                        .get_value()
                                        .into_iter()
                                        .map(|(class, cell)| {
                                            let value = cell.run(row.clone());
                                            view! { <TableCell class=class>{value}</TableCell> }
                                        })
                                        .collect_view()}
                                </TableRow>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </TableBody>
        </Table>
    }
}

#[component]
pub fn DataTableSelectionCount(
    selection: TableSelection,
    #[prop(into)] total: Signal<usize>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <p class=move || cn!("text-sm text-muted-foreground", class.get())>
            {move || {
                format!("{} of {} row(s) selected.", selection.count(), total.get())
            }}
        </p>
    }
}
