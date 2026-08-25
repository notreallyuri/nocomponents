//! Data table blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        data_table::{DataTable, DataTableColumn},
        pagination::{
            Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationNext,
            PaginationPrevious,
        },
    },
    primitives::table::{SortDirection, TableSelection, TableSort},
};
use std::cmp::Ordering;

const PAGED: &str = r#"let sort = TableSort::new();
let selection = TableSelection::new();
let page = RwSignal::new(1usize);

// Sort the whole set, then take the page — not the other way round. The table sorts
// what it is handed, so paging first would order each page against itself.
let sorted = Memo::new(move |_| {
    let mut rows = INVOICES.to_vec();
    let Some(column) = sort.column.get() else { return rows };
    let descending = sort.direction.get() == SortDirection::Descending;
    rows.sort_by(|a, b| {
        let order = compare(&column, a, b);
        if descending { order.reverse() } else { order }
    });
    rows
});

let visible = Signal::derive(move || {
    sorted.get().into_iter().skip((page.get() - 1) * PAGE_SIZE).take(PAGE_SIZE).collect()
});

// A new order is a new first page; staying on page three of a re-sorted list
// shows rows that were nowhere near it a moment ago.
Effect::new(move |_| {
    sort.column.track();
    sort.direction.track();
    page.set(1);
});

view! {
    <DataTable
        rows=visible
        columns=columns
        row_id=Callback::new(|row: Invoice| row.id.to_string())
        selection=selection
        sort=sort
    />
    <Pagination>…</Pagination>
}"#;

#[derive(Clone, PartialEq)]
struct Invoice {
    id: &'static str,
    customer: &'static str,
    status: &'static str,
    amount: f64,
}

const PAGE_SIZE: usize = 5;

// One row per line, which is what a table of data should look like.
#[rustfmt::skip]
const INVOICES: &[Invoice] = &[
    Invoice { id: "INV-1001", customer: "Acme Corp", status: "Paid", amount: 1250.00 },
    Invoice { id: "INV-1002", customer: "Globex", status: "Pending", amount: 480.50 },
    Invoice { id: "INV-1003", customer: "Initech", status: "Paid", amount: 3200.00 },
    Invoice { id: "INV-1004", customer: "Umbrella", status: "Overdue", amount: 150.75 },
    Invoice { id: "INV-1005", customer: "Hooli", status: "Paid", amount: 920.00 },
    Invoice { id: "INV-1006", customer: "Soylent", status: "Pending", amount: 2750.25 },
    Invoice { id: "INV-1007", customer: "Stark Industries", status: "Paid", amount: 6100.00 },
    Invoice { id: "INV-1008", customer: "Wayne Enterprises", status: "Overdue", amount: 75.00 },
    Invoice { id: "INV-1009", customer: "Cyberdyne", status: "Paid", amount: 1875.40 },
    Invoice { id: "INV-1010", customer: "Tyrell Corp", status: "Pending", amount: 410.00 },
    Invoice { id: "INV-1011", customer: "Aperture Science", status: "Paid", amount: 2240.60 },
    Invoice { id: "INV-1012", customer: "Black Mesa", status: "Overdue", amount: 995.00 },
    Invoice { id: "INV-1013", customer: "Weyland-Yutani", status: "Paid", amount: 5300.00 },
];

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-page-at-a-time",
    title: "A page at a time",
    uses: &["badge", "data-table", "pagination"],
    description: "`DataTable` shows what it is handed, so paging is the caller's: sort the whole \
                  set, take a slice of it, hand that over. Doing it the other way round — page \
                  first, sort second — orders each page against itself, which looks like sorting \
                  right up until you turn to page two. The selection is the other seam: it is a \
                  handle held out here rather than a context inside the table, so it counts across \
                  pages, while the header's own box only ever ticks the rows in front of you.",
    code: PAGED,
    view: || view! { <PagedTable /> }.into_any(),
}];

fn compare(column: &str, a: &Invoice, b: &Invoice) -> Ordering {
    match column {
        "customer" => a.customer.cmp(b.customer),
        "amount" => a.amount.partial_cmp(&b.amount).unwrap_or(Ordering::Equal),
        _ => a.id.cmp(b.id),
    }
}

fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Paid" => BadgeVariant::Secondary,
        "Overdue" => BadgeVariant::Destructive,
        _ => BadgeVariant::Outline,
    }
}

#[component]
fn PagedTable() -> impl IntoView {
    let sort = TableSort::new();
    let selection = TableSelection::new();
    let page = RwSignal::new(1usize);

    let sorted = Memo::new(move |_| {
        let mut rows = INVOICES.to_vec();
        let Some(column) = sort.column.get() else {
            return rows;
        };
        let descending = sort.direction.get() == SortDirection::Descending;
        rows.sort_by(|a, b| {
            let order = compare(&column, a, b);
            if descending { order.reverse() } else { order }
        });
        rows
    });

    let pages = Signal::derive(move || sorted.get().len().div_ceil(PAGE_SIZE).max(1));
    let visible = Signal::derive(move || {
        sorted
            .get()
            .into_iter()
            .skip((page.get() - 1) * PAGE_SIZE)
            .take(PAGE_SIZE)
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        sort.column.track();
        sort.direction.track();
        page.set(1);
    });

    let columns = vec![
        DataTableColumn::new(
            "invoice",
            "Invoice",
            Callback::new(|row: Invoice| {
                view! { <span class="font-medium">{row.id}</span> }.into_any()
            }),
        )
        .sortable(Callback::new(|(a, b): (Invoice, Invoice)| a.id.cmp(b.id))),
        DataTableColumn::new(
            "customer",
            "Customer",
            Callback::new(|row: Invoice| view! { {row.customer} }.into_any()),
        )
        .sortable(Callback::new(|(a, b): (Invoice, Invoice)| {
            a.customer.cmp(b.customer)
        })),
        DataTableColumn::new(
            "status",
            "Status",
            Callback::new(|row: Invoice| {
                view! { <Badge variant=status_variant(row.status)>{row.status}</Badge> }.into_any()
            }),
        ),
        DataTableColumn::new(
            "amount",
            "Amount",
            Callback::new(|row: Invoice| {
                view! {
                    <span class="font-mono tabular-nums">{format!("${:.2}", row.amount)}</span>
                }
                .into_any()
            }),
        )
        .sortable(Callback::new(|(a, b): (Invoice, Invoice)| {
            a.amount.partial_cmp(&b.amount).unwrap_or(Ordering::Equal)
        }))
        .class("text-right"),
    ];

    view! {
        <div class="flex flex-col gap-4">
            <DataTable
                rows=visible
                columns=columns
                row_id=Callback::new(|row: Invoice| row.id.to_string())
                selection=selection
                sort=sort
            />

            <div class="flex flex-wrap items-center justify-between gap-3">
                <p class="text-sm text-muted-foreground">
                    {move || match selection.count() {
                        0 => format!("{} invoices", INVOICES.len()),
                        picked => format!("{picked} of {} selected", INVOICES.len()),
                    }}
                </p>

                <Pagination class="mx-0 w-auto justify-end">
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious
                                href="#"
                                class=move || {
                                    if page.get() == 1 {
                                        "pointer-events-none opacity-50"
                                    } else {
                                        ""
                                    }
                                }
                                on:click=move |e| {
                                    e.prevent_default();
                                    page.update(|current| {
                                        *current = current.saturating_sub(1).max(1);
                                    });
                                }
                            />
                        </PaginationItem>

                        {move || {
                            (1..=pages.get())
                                .map(|number| {
                                    view! {
                                        <PaginationItem>
                                            <PaginationLink
                                                href="#"
                                                active=move || page.get() == number
                                                on:click=move |e| {
                                                    e.prevent_default();
                                                    page.set(number);
                                                }
                                            >
                                                {number.to_string()}
                                            </PaginationLink>
                                        </PaginationItem>
                                    }
                                })
                                .collect_view()
                        }}

                        <PaginationItem>
                            <PaginationNext
                                href="#"
                                class=move || {
                                    if page.get() == pages.get() {
                                        "pointer-events-none opacity-50"
                                    } else {
                                        ""
                                    }
                                }
                                on:click=move |e| {
                                    e.prevent_default();
                                    let last = pages.get_untracked();
                                    page.update(|current| *current = (*current + 1).min(last));
                                }
                            />
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
            </div>
        </div>
    }
}
