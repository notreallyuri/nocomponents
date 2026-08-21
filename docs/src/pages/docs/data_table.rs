use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        data_table::{DataTable, DataTableColumn, DataTableSelectionCount},
        input::Input,
    },
    primitives::table::{SortDirection, TableSelection, TableSort},
};

#[derive(Clone, PartialEq)]
struct Payment {
    id: &'static str,
    email: &'static str,
    status: &'static str,
    amount: i64,
}

const PAYMENTS: &[Payment] = &[
    Payment {
        id: "m5gr84i9",
        email: "ken99@example.com",
        status: "success",
        amount: 31600,
    },
    Payment {
        id: "3u1reuv4",
        email: "abe45@example.com",
        status: "success",
        amount: 24200,
    },
    Payment {
        id: "derv1ws0",
        email: "monserrat44@example.com",
        status: "processing",
        amount: 83700,
    },
    Payment {
        id: "5kma53ae",
        email: "silas22@example.com",
        status: "failed",
        amount: 87400,
    },
    Payment {
        id: "bhqecj4p",
        email: "carmella@example.com",
        status: "success",
        amount: 72100,
    },
];

const DEFAULT: &str = r#"#[derive(Clone, PartialEq)]
struct Payment { id: &'static str, email: &'static str, status: &'static str, amount: i64 }

let columns = vec![
    DataTableColumn::new("email", "Email", Callback::new(|row: Payment| {
        view! { <span class="font-medium">{row.email}</span> }.into_any()
    }))
    .sortable(Callback::new(|(a, b): (Payment, Payment)| a.email.cmp(b.email))),

    DataTableColumn::new("amount", "Amount", Callback::new(|row: Payment| {
        view! { <span>{format!("£{:.2}", row.amount as f64 / 100.0)}</span> }.into_any()
    }))
    .sortable(Callback::new(|(a, b): (Payment, Payment)| a.amount.cmp(&b.amount)))
    .class("text-right"),
];

view! {
    <DataTable
        rows=Signal::derive(|| PAYMENTS.to_vec())
        columns=columns
        row_id=Callback::new(|row: Payment| row.id.to_string())
        selection=selection
        sort=TableSort::sorted_by("email", SortDirection::Ascending)
    />
}"#;

const HELPERS: &str = r#"// The two helpers are plain handles — no context, no root component — so they
// work just as well over a table you have laid out by hand.
let sort = TableSort::new();
sort.toggle("email");          // ascending → descending → unsorted
sort.aria_sort("email");       // what the `<th>` should announce

let selection = TableSelection::new();
selection.toggle("m5gr84i9");
selection.state(&page_ids);    // None | Some | All — `Some` is an indeterminate header box
selection.set_all(&page_ids, true);   // leaves rows on other pages alone"#;

const FILTERED: &str = r#"// Filtering stays outside: the table sorts what it is given and never
// touches the signal it was given it from.
let query = RwSignal::new(String::new());
let rows = Signal::derive(move || {
    let query = query.get().to_lowercase();
    PAYMENTS.iter().filter(|p| p.email.contains(&query)).cloned().collect::<Vec<_>>()
});"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "DataTable",
        description: "A table over a list of rows, with the sorting and the selecting wired up. Generic over the row type; it sorts a copy for display and never writes back to the signal it was handed.",
        props: &[
            Prop {
                name: "rows",
                ty: "Signal<Vec<T>>",
                default: "",
                description: "The rows to show. `T: Clone + PartialEq + Send + Sync`. Filter and paginate outside — this shows what it is given.",
            },
            Prop {
                name: "columns",
                ty: "Vec<DataTableColumn<T>>",
                default: "",
                description: "The columns, in order.",
            },
            Prop {
                name: "row_id",
                ty: "Option<Callback<T, String>>",
                default: "None",
                description: "What makes a row itself. Without it there is no checkbox column.",
            },
            Prop {
                name: "selection",
                ty: "Option<TableSelection>",
                default: "None",
                description: "Where the picked rows are kept. Pass one, with `row_id`, to get the checkbox column.",
            },
            Prop {
                name: "sort",
                ty: "Option<TableSort>",
                default: "None",
                description: "Which column is in force. Pass one to read it, preset it, or share it with something else; omitted, the table owns one.",
            },
            Prop {
                name: "empty",
                ty: "&'static str",
                default: "\"No results.\"",
                description: "What to say when there is nothing to show.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the table's classes.",
            },
        ],
    },
    ApiEntry {
        name: "DataTableColumn",
        description: "One column, built with `new(key, header, cell)` and then `.sortable(compare)` or `.class(…)`. Not a component — a value you put in a `Vec`.",
        props: &[
            Prop {
                name: "key",
                ty: "&'static str",
                default: "",
                description: "What this column is called in the sort state. Unique within the table.",
            },
            Prop {
                name: "header",
                ty: "&'static str",
                default: "",
                description: "The column's heading.",
            },
            Prop {
                name: "cell",
                ty: "Callback<T, AnyView>",
                default: "",
                description: "Turns a row into the cell's contents.",
            },
            Prop {
                name: "compare",
                ty: "Option<Callback<(T, T), Ordering>>",
                default: "None",
                description: "Passing one makes the header sortable. Give the ascending order; descending is its reverse. There is no way to derive an order for an arbitrary row, and sorting the rendered text would put \"10\" before \"9\".",
            },
            Prop {
                name: "class",
                ty: "&'static str",
                default: "\"\"",
                description: "Merged onto the header cell and every body cell, for alignment and width.",
            },
        ],
    },
    ApiEntry {
        name: "DataTableSelectionCount",
        description: "The \"3 of 12 row(s) selected.\" line that belongs under a table with a checkbox column.",
        props: &[
            Prop {
                name: "selection",
                ty: "TableSelection",
                default: "",
                description: "The same handle the table was given.",
            },
            Prop {
                name: "total",
                ty: "Signal<usize>",
                default: "",
                description: "How many rows there are to select from.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the line's classes.",
            },
        ],
    },
    ApiEntry {
        name: "TableSort",
        description: "Which column a table is sorted by, and which way. A plain handle from `primitives::table` — build one with `TableSort::new()` or `TableSort::sorted_by(column, direction)`.",
        props: &[
            Prop {
                name: "column",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The key of the column in force, or nothing for the order the rows arrived in.",
            },
            Prop {
                name: "direction",
                ty: "RwSignal<SortDirection>",
                default: "Ascending",
                description: "Which way that column is sorted.",
            },
            Prop {
                name: "toggle(column)",
                ty: "fn(&str)",
                default: "",
                description: "What clicking a header does: a new column sorts ascending, the same one flips, and flipping a descending column clears the sort — so there is always a way back to the original order.",
            },
            Prop {
                name: "aria_sort(column)",
                ty: "fn(&str) -> &'static str",
                default: "",
                description: "What that column's `<th>` should announce: \"ascending\", \"descending\" or \"none\".",
            },
        ],
    },
    ApiEntry {
        name: "TableSelection",
        description: "Which rows are picked, by whatever id you use for a row. Also from `primitives::table`, and just as usable over a table you laid out by hand.",
        props: &[
            Prop {
                name: "selected",
                ty: "RwSignal<Vec<String>>",
                default: "[]",
                description: "The picked ids.",
            },
            Prop {
                name: "toggle(id) / set(id, on)",
                ty: "fn(&str)",
                default: "",
                description: "Picks or unpicks one row.",
            },
            Prop {
                name: "state(ids)",
                ty: "fn(&[String]) -> SelectionState",
                default: "",
                description: "Whether none, some or all of those rows are picked. `Some` is what makes a header checkbox indeterminate.",
            },
            Prop {
                name: "set_all(ids, on)",
                ty: "fn(&[String], bool)",
                default: "",
                description: "Sets exactly those rows, leaving anything picked elsewhere alone — a select-all on page two must not silently drop page one.",
            },
        ],
    },
];

fn columns() -> Vec<DataTableColumn<Payment>> {
    vec![
        DataTableColumn::new(
            "email",
            "Email",
            Callback::new(|row: Payment| {
                view! { <span class="font-medium">{row.email}</span> }.into_any()
            }),
        )
        .sortable(Callback::new(|(a, b): (Payment, Payment)| {
            a.email.cmp(b.email)
        })),
        DataTableColumn::new(
            "status",
            "Status",
            Callback::new(|row: Payment| {
                let variant = match row.status {
                    "success" => BadgeVariant::Secondary,
                    "failed" => BadgeVariant::Destructive,
                    _ => BadgeVariant::Outline,
                };
                view! { <Badge variant=variant>{row.status}</Badge> }.into_any()
            }),
        )
        .sortable(Callback::new(|(a, b): (Payment, Payment)| {
            a.status.cmp(b.status)
        })),
        DataTableColumn::new(
            "amount",
            "Amount",
            Callback::new(|row: Payment| {
                view! {
                    <span class="font-mono">
                        {format!("£{:.2}", row.amount as f64 / 100.0)}
                    </span>
                }
                .into_any()
            }),
        )
        .sortable(Callback::new(|(a, b): (Payment, Payment)| {
            a.amount.cmp(&b.amount)
        }))
        .class("text-right"),
    ]
}

#[component]
pub fn Page() -> impl IntoView {
    let selection = TableSelection::new();
    let sort = TableSort::sorted_by("email", SortDirection::Ascending);

    let query = RwSignal::new(String::new());
    let rows = Signal::derive(move || {
        let query = query.get().to_lowercase();
        PAYMENTS
            .iter()
            .filter(|payment| query.is_empty() || payment.email.contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <DocLayout
            title="Data Table"
            description="A table over a list of rows, with the sorting and the selecting wired up."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Sort by clicking a header — a third click clears it, so the original order is never out of reach. The checkbox column appears once the table knows what makes a row itself, and the select-all box only ever acts on the rows on show."
                    code=DEFAULT
                >
                    <div class="flex flex-col gap-3">
                        <div class="flex items-center gap-2">
                            <Input
                                model=query
                                attr:placeholder="Filter emails…"
                                class="max-w-56"
                            />
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Sm
                                on:click=move |_| selection.clear()
                            >
                                "Clear selection"
                            </Button>
                        </div>

                        <div class="rounded-lg border">
                            <DataTable
                                rows=rows
                                columns=columns()
                                row_id=Callback::new(|row: Payment| row.id.to_string())
                                selection=selection
                                sort=sort
                                empty="No payments match that filter."
                            />
                        </div>

                        <DataTableSelectionCount
                            selection=selection
                            total=Signal::derive(move || rows.get().len())
                        />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Filtering stays outside"
                    description="The table shows what it is handed and sorts a copy of it. Filtering, paging and fetching are all the caller's — which is what lets the same table sit over a `Vec`, a page of results, or a server that does the sorting itself."
                    code=FILTERED
                >
                    <p class="text-sm text-muted-foreground">
                        "The filter above is a plain signal the rows are derived from; the table
                        never writes back to it. Sorting is currently: "
                        <span class="font-mono text-foreground">
                            {move || match sort.column.get() {
                                Some(column) => {
                                    format!("{column} {}", sort.direction.get().as_str())
                                }
                                None => "unsorted".to_string(),
                            }}
                        </span>
                    </p>
                </DemoSection>

                <DemoSection
                    title="The helpers on their own"
                    description="`TableSort` and `TableSelection` are plain handles rather than context providers — a table already has its rows and columns in one place, so passing the handle where it is needed beats hiding it in a context. Both work over a table you laid out by hand."
                    code=HELPERS
                >
                    <p class="text-sm text-muted-foreground">
                        "There is also a `TableSortHeaderRoot` in `primitives::table` that renders
                        the `<th>`, its `aria-sort` and the button that toggles it, leaving the
                        look to you."
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
