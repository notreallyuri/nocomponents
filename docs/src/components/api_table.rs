//! The API reference table.
//!
//! One row per prop, in the order the component declares them. The rows are data rather than
//! markup on purpose: this is the shape an extractor over `#[component]` signatures would emit,
//! so the tables can stop being written by hand without the pages changing.

use crate::components::page_nav::slug;
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
        // Deliberately a size up from a demo's heading: this is the page's second half, not
        // another demo in the list.
        <section class="mt-4 flex flex-col gap-5 border-t border-border/60 pt-8">
            <div class="flex flex-col gap-1">
                <h2
                    id="api-reference"
                    data-toc="section"
                    class="text-lg font-semibold tracking-tight text-foreground"
                >
                    "API reference"
                </h2>
                <p class="text-sm text-muted-foreground">
                    "Every prop each part takes. Anything not listed is forwarded to the rendered element."
                </p>
            </div>

            <div class="flex flex-col gap-6">
                {entries
                    .iter()
                    .map(|entry| {
                        view! {
                            // Parts sit under the one heading, ruled off from each other rather than
                            // spaced apart, so a component with a dozen of them still reads as one
                            // reference and not as a dozen sections.
                            <div class="flex flex-col gap-2 [&:not(:first-child)]:border-t [&:not(:first-child)]:border-border/60 [&:not(:first-child)]:pt-6">
                                <div class="flex flex-col gap-0.5">
                                    <h3
                                        id=slug(entry.name)
                                        data-toc="item"
                                        class="font-mono text-sm font-medium text-foreground"
                                    >
                                        {entry.name}
                                    </h3>
                                    <p class="text-xs text-muted-foreground">{entry.description}</p>
                                </div>

                                // The library's cells are `whitespace-nowrap`, which is right for
                                // data and wrong for prose; `table-fixed` makes the widths bind.
                                <Table class="table-fixed text-xs">
                                    <TableHeader>
                                        <TableRow>
                                            <TableHead class="w-[14%] whitespace-normal">
                                                "Prop"
                                            </TableHead>
                                            // Wide enough that the longest type in the library breaks at its comma rather
                                            // than mid-identifier, where a break reads as a typo.
                                            <TableHead class="w-[38%] whitespace-normal">
                                                "Type"
                                            </TableHead>
                                            <TableHead class="w-[11%] whitespace-normal">
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
            </div>
        </section>
    }
}
