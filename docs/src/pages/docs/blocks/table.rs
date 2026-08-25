//! Table blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        context_menu::{
            ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuLabel, ContextMenuPortal,
            ContextMenuSeparator, ContextMenuTrigger,
        },
        table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow},
    },
    primitives::context_menu::use_context_menu,
};

#[derive(Clone, PartialEq)]
struct Deployment {
    id: &'static str,
    env: &'static str,
    status: &'static str,
    when: &'static str,
}

#[rustfmt::skip]
const DEPLOYMENTS: &[Deployment] = &[
    Deployment { id: "dep_1042", env: "production", status: "Live",     when: "12 minutes ago" },
    Deployment { id: "dep_1041", env: "preview",    status: "Preview",  when: "an hour ago" },
    Deployment { id: "dep_1039", env: "preview",    status: "Preview",  when: "yesterday" },
    Deployment { id: "dep_1036", env: "production", status: "Rolled back", when: "3 days ago" },
];

const ROW_ACTIONS: &str = r##"// One menu for the whole table, not one per row. A context menu brings a
// wrapper element of its own, and a `<div>` between `<tbody>` and `<tr>` is
// not something the parser will keep — it hoists it out and the row goes with
// it. So the menu wraps the table, and *which row it is about* is state.
let target = RwSignal::new(None::<&'static str>);

view! {
    <ContextMenu>
        <ContextMenuTrigger>
            <Table>
                <TableHeader>
                    // A right-click that misses the rows is still a right-click on the
                    // table, so the header says so rather than leaving the last row named.
                    <TableRow on:contextmenu=move |_| target.set(None)>…</TableRow>
                </TableHeader>
                <TableBody>
                    // The row is in scope here, so nothing has to be read back out of the
                    // DOM: the row records itself on the way past, and the event carries on
                    // up to the trigger, which opens the menu where the pointer is.
                    <TableRow
                        selected=Signal::derive(move || menu.is_open.get() && target.get() == Some(id))
                        on:contextmenu=move |_| target.set(Some(id))
                    >…</TableRow>
                </TableBody>
            </Table>
        </ContextMenuTrigger>

        <ContextMenuPortal>
            <ContextMenuContent class="min-w-44">
                // What the menu holds is read off the target, so one menu covers the
                // rows and the table itself, and a deployment that is already live is
                // not offered a promotion.
                {move || match target.get().map(deployment) {
                    Some(row) => …row actions…,
                    None => …table actions…,
                }}
            </ContextMenuContent>
        </ContextMenuPortal>
    </ContextMenu>
}"##;

pub const BLOCKS: &[Block] = &[Block {
    slug: "right-click-a-row",
    title: "Right-click a row",
    uses: &["badge", "context-menu", "table"],
    description: "One menu for the whole table rather than one per row, and not to save memory: a \
                  context menu brings a wrapper element of its own, and a `<div>` sitting between \
                  `<tbody>` and `<tr>` is not something the HTML parser will keep — it hoists the \
                  element out of the table and the row goes with it. So the menu wraps the table, \
                  and which row it is about becomes state. The row records itself on the way \
                  past: the `contextmenu` event lands on the `<tr>` first, where the row is still \
                  in scope and can simply say so, and then carries on up to the trigger, which \
                  opens the menu at the pointer. Nothing is read back out of the DOM — no \
                  `closest('tr')`, no id parsed off a data attribute. Because the target is \
                  state, the menu's *contents* can be read off it too, so one menu serves both \
                  the rows and the table itself, and a deployment that is already live is not \
                  offered a promotion. And because the row knows it is the one being talked \
                  about, it can look it: `selected` on the row is true while the menu is open \
                  about it, which is the part that stops a menu floating over a table from being \
                  a menu about nothing in particular.",
    code: ROW_ACTIONS,
    view: || view! { <DeploymentTable /> }.into_any(),
}];

fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Live" => BadgeVariant::Default,
        "Rolled back" => BadgeVariant::Destructive,
        _ => BadgeVariant::Secondary,
    }
}

#[component]
fn DeploymentTable() -> impl IntoView {
    let rows = RwSignal::new(DEPLOYMENTS.to_vec());
    let target = RwSignal::new(None::<&'static str>);

    let set_status = move |id: &'static str, status: &'static str| {
        rows.update(|rows| {
            if let Some(row) = rows.iter_mut().find(|row| row.id == id) {
                row.status = status;
            }
        })
    };

    view! {
        <ContextMenu>
            <ContextMenuTrigger>
                <Rows rows=rows target=target />
            </ContextMenuTrigger>

            <ContextMenuPortal>
                <ContextMenuContent class="min-w-44">
                    {move || match target.get() {
                        Some(id) => {
                            let status = rows
                                .with(|rows| {
                                    rows.iter().find(|row| row.id == id).map(|row| row.status)
                                })
                                .unwrap_or("");
                            Some(

                                view! {
                                    <ContextMenuLabel>{id}</ContextMenuLabel>
                                    <ContextMenuSeparator />
                                    {(status == "Preview")
                                        .then(|| {
                                            view! {
                                                <ContextMenuItem on_click=Callback::new(move |_| {
                                                    set_status(id, "Live")
                                                })>"Promote to production"</ContextMenuItem>
                                            }
                                        })}
                                    {(status == "Live")
                                        .then(|| {
                                            view! {
                                                <ContextMenuItem on_click=Callback::new(move |_| {
                                                    set_status(id, "Rolled back")
                                                })>"Roll back"</ContextMenuItem>
                                            }
                                        })}
                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                        set_status(id, "Building")
                                    })>"Re-run"</ContextMenuItem>
                                    <ContextMenuSeparator />
                                    <ContextMenuItem
                                        class="text-destructive"
                                        on_click=Callback::new(move |_| {
                                            rows.update(|rows| rows.retain(|row| row.id != id));
                                            target.set(None);
                                        })
                                    >
                                        "Delete"
                                    </ContextMenuItem>
                                }
                                    .into_any(),
                            )
                        }
                        None => {
                            Some(
                                view! {
                                    <ContextMenuLabel>"All deployments"</ContextMenuLabel>
                                    <ContextMenuSeparator />
                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                        rows.set(DEPLOYMENTS.to_vec())
                                    })>"Reset the table"</ContextMenuItem>
                                }
                                    .into_any(),
                            )
                        }
                    }}
                </ContextMenuContent>
            </ContextMenuPortal>
        </ContextMenu>
    }
}

/// The table itself, a component of its own so it can read the menu's context: a row that is what
/// the open menu is about says so, and stops saying it when the menu goes.
#[component]
fn Rows(rows: RwSignal<Vec<Deployment>>, target: RwSignal<Option<&'static str>>) -> impl IntoView {
    let menu = use_context_menu();

    view! {
        <Table>
            <TableCaption>"Right-click a row — or the header."</TableCaption>
            <TableHeader>
                // A right-click that misses the rows is still a right-click on the table, so the
                // header says so rather than leaving the last row named.
                <TableRow on:contextmenu=move |_| target.set(None)>
                    <TableHead>"Deployment"</TableHead>
                    <TableHead>"Environment"</TableHead>
                    <TableHead>"Status"</TableHead>
                    <TableHead class="text-right">"When"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {move || {
                    rows.get()
                        .into_iter()
                        .map(|row| {
                            let Deployment { id, env, status, when } = row;

                            view! {
                                <TableRow
                                    selected=Signal::derive(move || {
                                        menu.is_open.get() && target.get() == Some(id)
                                    })
                                    on:contextmenu=move |_| target.set(Some(id))
                                >
                                    <TableCell class="font-mono text-xs">{id}</TableCell>
                                    <TableCell class="text-muted-foreground">{env}</TableCell>
                                    <TableCell>
                                        <Badge variant=status_variant(status)>{status}</Badge>
                                    </TableCell>
                                    <TableCell class="text-right text-muted-foreground">
                                        {when}
                                    </TableCell>
                                </TableRow>
                            }
                        })
                        .collect_view()
                }}
            </TableBody>
        </Table>
    }
}
