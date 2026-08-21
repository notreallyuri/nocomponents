//! The API reference table.
//!
//! One row per prop, in the order the component declares them. The rows are data rather than
//! markup on purpose: this is the shape an extractor over `#[component]` signatures would emit,
//! so the tables can stop being written by hand without the pages changing.

use leptos::prelude::*;
use nocomponents::components::{
    code::Code,
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};

/// One prop of one component.
pub struct Prop {
    pub name: &'static str,
    pub ty: &'static str,
    /// What the prop resolves to when it is not passed. `""` for a required prop.
    pub default: &'static str,
    pub description: &'static str,
}

/// The props of one component, under the name it is called by.
pub struct ApiEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub props: &'static [Prop],
}

/// The reference for every part of a component, below its demos.
#[component]
pub fn ApiReference(entries: &'static [ApiEntry]) -> impl IntoView {
    view! {
        <section class="flex flex-col gap-6">
            <div class="flex flex-col gap-0.5">
                <h2 class="text-sm font-semibold text-foreground">"API reference"</h2>
                <p class="text-xs text-muted-foreground">
                    "Every prop each part takes. Anything not listed is forwarded to the rendered element."
                </p>
            </div>

            {entries
                .iter()
                .map(|entry| {
                    view! {
                        <div class="flex flex-col gap-2">
                            <div class="flex flex-col gap-0.5">
                                <h3 class="font-mono text-sm font-medium text-foreground">
                                    {entry.name}
                                </h3>
                                <p class="text-xs text-muted-foreground">{entry.description}</p>
                            </div>

                            // The library's cells are `whitespace-nowrap`, which is right for
                            // data and wrong for prose; `table-fixed` makes the widths bind.
                            <Table class="table-fixed text-xs">
                                <TableHeader>
                                    <TableRow>
                                        <TableHead class="w-[18%] whitespace-normal">
                                            "Prop"
                                        </TableHead>
                                        <TableHead class="w-[30%] whitespace-normal">
                                            "Type"
                                        </TableHead>
                                        <TableHead class="w-[12%] whitespace-normal">
                                            "Default"
                                        </TableHead>
                                        <TableHead class="whitespace-normal">
                                            "Description"
                                        </TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    {entry
                                        .props
                                        .iter()
                                        .map(|prop| {
                                            view! {
                                                <TableRow>
                                                    <TableCell class="align-top whitespace-normal">
                                                        <Code>{prop.name}</Code>
                                                    </TableCell>
                                                    <TableCell class="align-top break-words whitespace-normal text-muted-foreground">
                                                        <span class="font-mono">{prop.ty}</span>
                                                    </TableCell>
                                                    <TableCell class="align-top font-mono whitespace-normal text-muted-foreground">
                                                        {if prop.default.is_empty() { "—" } else { prop.default }}
                                                    </TableCell>
                                                    <TableCell class="align-top whitespace-normal text-muted-foreground">
                                                        {prop.description}
                                                    </TableCell>
                                                </TableRow>
                                            }
                                        })
                                        .collect_view()}
                                </TableBody>
                            </Table>
                        </div>
                    }
                })
                .collect_view()}
        </section>
    }
}
