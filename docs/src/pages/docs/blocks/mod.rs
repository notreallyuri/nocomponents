//! Blocks: the compositions, gathered by what they *are* rather than by whose page they sit on.
//!
//! A component's own page answers "what does this part do", and grows a demo per prop. That is the
//! wrong place for "here is a colour picker inside a popover inside a dialog" — the interesting
//! part of a block is the *seam* between components, and putting one on a component page would
//! bury the API under scenery.
//!
//! They used to live at `/docs/<component>/blocks`, which meant every block was filed under one
//! owner. That was always a fiction: each of these composes between three and ten components, so
//! the owner was whichever one the file was named after, and the other nine could not reach it.
//! They live at `/blocks` now, and each carries the list of components it is built from — so a
//! block is reachable from every component in it, and the index is searchable across all of them.
//!
//! The blocks themselves are a file per subject, since a worked example runs to a hundred lines of
//! markup and a snippet beside it. Each exposes `pub const BLOCKS: &[Block]`; adding one means a
//! module here and a line in [`REGISTRY`], and adding another to a file that exists means touching
//! nothing else at all.

mod canvas;
mod chart;
mod color_picker;
mod combobox;
mod command;
mod data_table;
mod dialog;
mod field;
mod image_cropper;
mod kanban;
mod sidebar;
mod table;
mod toast;

use crate::{
    app::href,
    components::demo_section::DemoSection,
    layout::doc_layout::{DocLayout, NAV},
};
use leptos::{either::Either, prelude::*};
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};
use nocomponents::components::{
    empty::{Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle},
    input::Input,
};

/// One worked example. Not a demo of a prop — a composition, with the seams showing.
pub struct Block {
    /// The last segment of `/blocks/<slug>`.
    pub slug: &'static str,
    pub title: &'static str,
    /// The components it is built out of, as they are spelled in `/docs/<slug>`.
    ///
    /// What makes a block findable from more than one place. Every one of these composes between
    /// three and ten components, so filing it under a single owner — which is what
    /// `/docs/<component>/blocks` did — picks one of them and hides it from the rest.
    pub uses: &'static [&'static str],
    pub description: &'static str,
    /// The markup, in a module-level const like every other snippet in the docs. leptosfmt
    /// flattens raw strings written inline in a `view!`.
    pub code: &'static str,
    /// A plain `fn` rather than a closure, so the whole registry stays a `const`.
    pub view: fn() -> AnyView,
}

/// Every block there is. A flat list rather than a map keyed by component: a block belongs to all
/// of the components it uses, and [`Block::uses`] is where that is written down.
const REGISTRY: &[&[Block]] = &[
    canvas::BLOCKS,
    chart::BLOCKS,
    color_picker::BLOCKS,
    combobox::BLOCKS,
    command::BLOCKS,
    data_table::BLOCKS,
    dialog::BLOCKS,
    field::BLOCKS,
    image_cropper::BLOCKS,
    kanban::BLOCKS,
    sidebar::BLOCKS,
    table::BLOCKS,
    toast::BLOCKS,
];

pub fn all() -> impl Iterator<Item = &'static Block> {
    REGISTRY.iter().copied().flatten()
}

fn find(slug: &str) -> Option<&'static Block> {
    all().find(|block| block.slug == slug)
}

/// A component's label, as `NAV` spells it. `None` for one that has no page.
fn label_for(slug: &str) -> Option<&'static str> {
    let path = format!("/docs/{slug}");
    NAV.iter()
        .flat_map(|group| group.items)
        .find(|item| item.href == path)
        .map(|item| item.label)
}

/// Whether a block answers to `query` — its title, its blurb, or any component in it.
///
/// Over the components too, and not only the prose, because "which blocks use a drawer" is the
/// question the old per-component filing could answer and a plain text search could not.
fn matches(block: &Block, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }

    block.title.to_lowercase().contains(&query)
        || block.description.to_lowercase().contains(&query)
        || block.uses.iter().any(|component| {
            component.contains(&query)
                || label_for(component).is_some_and(|label| label.to_lowercase().contains(&query))
        })
}

/// The components a block is built from, each linking to its own page.
#[component]
fn Uses(block: &'static Block) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-1.5">
            {block
                .uses
                .iter()
                .map(|component| {
                    let label = label_for(component).unwrap_or(component);
                    view! {
                        <A
                            href=href(&format!("/docs/{component}"))
                            attr:class="rounded-md border px-1.5 py-0.5 text-[0.7rem] text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                        >
                            {label}
                        </A>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// `/blocks` — everything there is, searchable.
#[component]
pub fn Index() -> impl IntoView {
    let query = use_query_map();
    // Seeded from `?uses=`, so a component page can hand the reader a filtered index rather than
    // a list they have to search themselves.
    let search = RwSignal::new(query.read_untracked().get("uses").unwrap_or_default());

    let showing = Signal::derive(move || {
        let search = search.get();
        all()
            .filter(|block| matches(block, &search))
            .collect::<Vec<_>>()
    });

    view! {
        <DocLayout
            title="Blocks"
            description="Whole pieces of interface, built out of several components at once."
        >
            <div class="flex flex-col gap-6">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "A component's page shows what one part does. These show several of them doing
                    a job together, which is where the interesting decisions are — and each one
                    says what it is built from, so you can start from a component and find every
                    block that uses it."
                </p>

                <Input
                    model=search
                    attr:placeholder="Search blocks, or a component in one…"
                    class="max-w-sm"
                />

                {move || {
                    let blocks = showing.get();
                    if blocks.is_empty() {
                        return Either::Left(
                            view! {
                                <Empty class="py-16">
                                    <EmptyHeader>
                                        <EmptyMedia variant=EmptyMediaVariant::Icon>
                                            <BlocksMark />
                                        </EmptyMedia>
                                        <EmptyTitle>"Nothing matches"</EmptyTitle>
                                        <EmptyDescription>
                                            "No block has that in its name, its description or its parts."
                                        </EmptyDescription>
                                    </EmptyHeader>
                                </Empty>
                            },
                        );
                    }
                    Either::Right(

                        view! {
                            <div class="grid gap-3 sm:grid-cols-2">
                                {blocks
                                    .into_iter()
                                    .map(|block| {
                                        view! {
                                            <A
                                                href=href(&format!("/blocks/{}", block.slug))
                                                attr:class="flex flex-col gap-2 rounded-lg border p-4 transition-colors hover:bg-accent/40"
                                            >
                                                <span class="text-sm font-semibold text-foreground">
                                                    {block.title}
                                                </span>
                                                <span class="line-clamp-3 text-xs leading-relaxed text-muted-foreground">
                                                    {block.description}
                                                </span>
                                                <Uses block=block />
                                            </A>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        },
                    )
                }}
            </div>
        </DocLayout>
    }
}

/// `/blocks/<slug>` — one of them, running.
#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    // Read reactively: the route is shared by every block, so moving between two of them changes
    // the parameter without remounting this.
    let slug = Signal::derive(move || params.read().get("slug").unwrap_or_default());

    view! {
        <DocLayout
            title="Blocks"
            description="Whole pieces of interface, built out of several components at once."
        >
            {move || match find(&slug.get()) {
                None => {
                    Either::Left(
                        view! {
                            <Empty class="py-16">
                                <EmptyHeader>
                                    <EmptyMedia variant=EmptyMediaVariant::Icon>
                                        <BlocksMark />
                                    </EmptyMedia>
                                    <EmptyTitle>"No such block"</EmptyTitle>
                                    <EmptyDescription>
                                        "It may have been renamed. "
                                        <A
                                            href=href("/blocks")
                                            attr:class="font-medium text-foreground underline underline-offset-4"
                                        >
                                            "All of them are here"
                                        </A> "."
                                    </EmptyDescription>
                                </EmptyHeader>
                            </Empty>
                        },
                    )
                }
                Some(block) => {
                    Either::Right(
                        view! {
                            <div class="flex flex-col gap-6">
                                <div class="flex flex-col gap-3">
                                    <A
                                        href=href("/blocks")
                                        attr:class="w-fit text-xs text-muted-foreground underline underline-offset-4 hover:text-foreground"
                                    >
                                        "← All blocks"
                                    </A>
                                    <Uses block=block />
                                </div>

                                <DemoSection
                                    title=block.title
                                    description=block.description
                                    code=block.code
                                >
                                    {(block.view)()}
                                </DemoSection>
                            </div>
                        },
                    )
                }
            }}
        </DocLayout>
    }
}

/// Three stacked panes, which is what a block is: several components in one arrangement.
#[component]
fn BlocksMark() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="square"
            stroke-linejoin="miter"
            class="size-6"
        >
            <rect width="18" height="7" x="3" y="3" rx="1" />
            <rect width="9" height="7" x="3" y="14" rx="1" />
            <rect width="5" height="7" x="16" y="14" rx="1" />
        </svg>
    }
}
