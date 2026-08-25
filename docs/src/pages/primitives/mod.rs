//! Primitives: the behaviour layer, documented where it can be found.
//!
//! Its own section rather than a page per entry in the sidebar, for the reason blocks got one: a
//! nav that lists fifty more names is a nav nobody reads. One line in the sidebar, an index behind
//! it, and a page each — and a component page links across to its own primitive, since nobody
//! looks up `DialogContentRoot` except while reading about `Dialog`.
//!
//! **These five have no styled layer at all**, which is why they are first. `drag`, `floating`,
//! `roving_focus`, `dismiss` and `typeahead` are what the other components are built *out of*, so
//! they have never had a page to be documented on — and a scheme that hung primitives off their
//! component pages could not have given them one.
//!
//! What a page here carries that a component page does not: the context it publishes, the
//! `data-*` attributes it writes, and the keys it binds. Those are the contract someone styling
//! the unstyled layer is working against, and none of them appear in a prop table.

mod dismiss;
mod drag;
mod floating;
mod roving_focus;
mod typeahead;

use crate::{app::href, layout::doc_layout::DocLayout};
use leptos::{either::Either, prelude::*};
use leptos_router::{components::A, hooks::use_params_map};
use nocomponents::components::{
    empty::{Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle},
    input::Input,
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};

/// One behaviour, and the components built on it.
pub struct Primitive {
    pub slug: &'static str,
    pub name: &'static str,
    /// One line, for the index.
    pub blurb: &'static str,
    /// The components that use it, as `/docs/<slug>` spells them. The other direction of a block's
    /// `uses`: a primitive is found from the thing that needed it as often as the other way round.
    pub used_by: &'static [&'static str],
    pub view: fn() -> AnyView,
}

const REGISTRY: &[&Primitive] = &[
    &dismiss::PRIMITIVE,
    &drag::PRIMITIVE,
    &floating::PRIMITIVE,
    &roving_focus::PRIMITIVE,
    &typeahead::PRIMITIVE,
];

pub fn all() -> impl Iterator<Item = &'static Primitive> {
    REGISTRY.iter().copied()
}

fn find(slug: &str) -> Option<&'static Primitive> {
    all().find(|primitive| primitive.slug == slug)
}

fn matches(primitive: &Primitive, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || primitive.name.to_lowercase().contains(&query)
        || primitive.blurb.to_lowercase().contains(&query)
        || primitive.used_by.iter().any(|c| c.contains(&query))
}

/// One row of an API: a name, what it is, and what it does.
///
/// Not `ApiEntry`: that is props, with a default column. Half of what is on these pages is methods
/// on a handle and fields on a value, and a default means nothing for either.
pub struct Signature {
    pub name: &'static str,
    pub shape: &'static str,
    pub what: &'static str,
}

#[component]
pub fn Signatures(
    title: &'static str,
    #[prop(optional)] note: Option<&'static str>,
    rows: &'static [Signature],
) -> impl IntoView {
    view! {
        <section class="flex flex-col gap-2">
            <div class="flex flex-col gap-0.5">
                <h3 class="font-mono text-sm font-medium text-foreground">{title}</h3>
                {note.map(|note| view! { <p class="text-xs text-muted-foreground">{note}</p> })}
            </div>

            <Table class="table-fixed text-xs">
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-[22%] whitespace-normal">"Name"</TableHead>
                        <TableHead class="w-[34%] whitespace-normal">"Shape"</TableHead>
                        <TableHead class="whitespace-normal">"What it does"</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {rows
                        .iter()
                        .map(|row| {
                            view! {
                                <TableRow>
                                    <TableCell class="align-top font-mono whitespace-normal text-foreground">
                                        {row.name}
                                    </TableCell>
                                    <TableCell class="align-top font-mono break-words whitespace-normal text-muted-foreground">
                                        {row.shape}
                                    </TableCell>
                                    <TableCell class="align-top whitespace-normal text-muted-foreground">
                                        {row.what}
                                    </TableCell>
                                </TableRow>
                            }
                        })
                        .collect_view()}
                </TableBody>
            </Table>
        </section>
    }
}

/// The components built on a primitive, each linking to its page.
#[component]
pub fn UsedBy(primitive: &'static Primitive) -> impl IntoView {
    view! {
        <Show when=move || !primitive.used_by.is_empty()>
            <div class="flex flex-wrap items-center gap-1.5">
                <span class="text-xs text-muted-foreground">"Built on by"</span>
                {primitive
                    .used_by
                    .iter()
                    .map(|component| {
                        view! {
                            <A
                                href=href(&format!("/docs/{component}"))
                                attr:class="rounded-md border px-1.5 py-0.5 text-[0.7rem] text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                            >
                                {*component}
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </Show>
    }
}

/// `/primitives` — the behaviour layer, searchable.
#[component]
pub fn Index() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let showing = Signal::derive(move || {
        let search = search.get();
        all()
            .filter(|primitive| matches(primitive, &search))
            .collect::<Vec<_>>()
    });

    view! {
        <DocLayout
            title="Primitives"
            description="The behaviour layer: state, ARIA and keys, with no classes on any of it."
        >
            <div class="flex flex-col gap-6">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "Every styled component is a primitive with classes on it. These are the
                    primitives that have no styled layer at all — the ones the others are built out
                    of — so this is the only place they are written down. The rest are documented
                    beside the component that wears them."
                </p>

                <Input
                    model=search
                    attr:placeholder="Search the behaviour layer…"
                    class="max-w-sm"
                />

                {move || {
                    let found = showing.get();
                    if found.is_empty() {
                        return Either::Left(
                            view! {
                                <Empty class="py-16">
                                    <EmptyHeader>
                                        <EmptyMedia variant=EmptyMediaVariant::Icon>
                                            <LayersMark />
                                        </EmptyMedia>
                                        <EmptyTitle>"Nothing matches"</EmptyTitle>
                                        <EmptyDescription>
                                            "No primitive has that in its name, its blurb or the components on it."
                                        </EmptyDescription>
                                    </EmptyHeader>
                                </Empty>
                            },
                        );
                    }
                    Either::Right(

                        view! {
                            <div class="grid gap-3 sm:grid-cols-2">
                                {found
                                    .into_iter()
                                    .map(|primitive| {
                                        view! {
                                            <A
                                                href=href(&format!("/primitives/{}", primitive.slug))
                                                attr:class="flex flex-col gap-2 rounded-lg border p-4 transition-colors hover:bg-accent/40"
                                            >
                                                <span class="font-mono text-sm font-semibold text-foreground">
                                                    {primitive.name}
                                                </span>
                                                <span class="text-xs leading-relaxed text-muted-foreground">
                                                    {primitive.blurb}
                                                </span>
                                                <span class="text-[0.7rem] text-muted-foreground">
                                                    {format!("{} components", primitive.used_by.len())}
                                                </span>
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

/// `/primitives/<slug>` — one of them.
#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let slug = Signal::derive(move || params.read().get("slug").unwrap_or_default());

    view! {
        <DocLayout
            title="Primitives"
            description="The behaviour layer: state, ARIA and keys, with no classes on any of it."
        >
            {move || match find(&slug.get()) {
                None => {
                    Either::Left(
                        view! {
                            <Empty class="py-16">
                                <EmptyHeader>
                                    <EmptyMedia variant=EmptyMediaVariant::Icon>
                                        <LayersMark />
                                    </EmptyMedia>
                                    <EmptyTitle>"No such primitive"</EmptyTitle>
                                    <EmptyDescription>
                                        "Only the five with no styled layer have pages so far. "
                                        <A
                                            href=href("/primitives")
                                            attr:class="font-medium text-foreground underline underline-offset-4"
                                        >
                                            "They are here"
                                        </A> "."
                                    </EmptyDescription>
                                </EmptyHeader>
                            </Empty>
                        },
                    )
                }
                Some(primitive) => {
                    Either::Right(
                        view! {
                            <div class="flex flex-col gap-8">
                                <div class="flex flex-col gap-3">
                                    <A
                                        href=href("/primitives")
                                        attr:class="w-fit text-xs text-muted-foreground underline underline-offset-4 hover:text-foreground"
                                    >
                                        "← All primitives"
                                    </A>
                                    <UsedBy primitive=primitive />
                                </div>
                                {(primitive.view)()}
                            </div>
                        },
                    )
                }
            }}
        </DocLayout>
    }
}

/// Stacked planes: a behaviour with layers of styling that may or may not be put on it.
#[component]
fn LayersMark() -> impl IntoView {
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
            <path d="m12 2 9 5-9 5-9-5 9-5Z" />
            <path d="m3 12 9 5 9-5" />
            <path d="m3 17 9 5 9-5" />
        </svg>
    }
}
