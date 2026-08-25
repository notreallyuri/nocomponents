//! Blocks: the compositions, one page per component.
//!
//! A component's own page answers "what does this part do", and grows a demo per prop. That is the
//! wrong place for "here is a colour picker inside a popover inside a dialog" — the interesting
//! part of a block is the *seam* between components, and putting those on the component pages
//! would bury the API under scenery and duplicate every block across the three components it
//! touches.
//!
//! So blocks live at `/docs/<component>/blocks`, off one parameterised route: [`Page`] looks its
//! component up in [`REGISTRY`], so a component gets a blocks page by existing and there is no
//! per-component route, nav entry or index line to forget. Everything not listed shows the empty
//! state.
//!
//! The blocks themselves are a file per component, since a worked example runs to a hundred lines
//! of markup and a snippet beside it — one file for all of them was pleasant at nought and
//! unreadable at four. Each exposes `pub const BLOCKS: &[Block]`; adding a component means a
//! module here and a line in [`REGISTRY`], and adding a block to one that already has a file means
//! touching nothing else at all.

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
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};
use nocomponents::components::empty::{
    Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};

/// One worked example. Not a demo of a prop — a composition, with the seams showing.
pub struct Block {
    pub title: &'static str,
    pub description: &'static str,
    /// The markup, in a module-level const like every other snippet in the docs. leptosfmt
    /// flattens raw strings written inline in a `view!`.
    pub code: &'static str,
    /// A plain `fn` rather than a closure, so the whole registry stays a `const`.
    pub view: fn() -> AnyView,
}

/// Blocks by component slug — the last segment of the `/docs/<slug>` its component's page lives
/// at. Add an entry here and the page fills in; nothing else needs touching.
const REGISTRY: &[(&str, &[Block])] = &[
    ("chart", chart::BLOCKS),
    ("color-picker", color_picker::BLOCKS),
    ("combobox", combobox::BLOCKS),
    ("command", command::BLOCKS),
    ("data-table", data_table::BLOCKS),
    ("dialog", dialog::BLOCKS),
    ("field", field::BLOCKS),
    ("image-cropper", image_cropper::BLOCKS),
    ("kanban", kanban::BLOCKS),
    ("sidebar", sidebar::BLOCKS),
    ("table", table::BLOCKS),
    ("toast", toast::BLOCKS),
];

/// The component this page belongs to, as it is spelled in `NAV`.
fn label_for(slug: &str) -> Option<&'static str> {
    let path = format!("/docs/{slug}");

    NAV.iter()
        .flat_map(|group| group.items)
        .find(|item| item.href == path)
        .map(|item| item.label)
}

fn blocks_for(slug: &str) -> &'static [Block] {
    REGISTRY
        .iter()
        .find(|(component, _)| *component == slug)
        .map(|(_, blocks)| *blocks)
        .unwrap_or(&[])
}

#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();

    // Read reactively rather than resolved once: the route is shared by every component, so
    // moving between two blocks pages changes the parameter without remounting this.
    let slug = Signal::derive(move || params.read().get("component").unwrap_or_default());

    view! {
        <DocLayout
            title="Blocks"
            description="Whole pieces of interface, built out of several components at once."
        >
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-2">
                    <h2 class="text-lg font-semibold tracking-tight text-foreground">
                        {move || match label_for(&slug.get()) {
                            Some(label) => format!("{label} blocks"),
                            None => "Blocks".to_string(),
                        }}
                    </h2>
                    <p class="text-sm text-muted-foreground">
                        "Examples of this component doing a real job alongside others — the kind of
                        thing that would swamp its own page. Its API and its behaviour are "
                        <A
                            href=move || href(&format!("/docs/{}", slug.get()))
                            attr:class="font-medium text-foreground underline underline-offset-4"
                        >
                            "on the component page"
                        </A> "."
                    </p>
                </div>

                {move || {
                    let blocks = blocks_for(&slug.get());
                    if blocks.is_empty() {
                        return view! {
                            <Empty class="py-16">
                                <EmptyHeader>
                                    <EmptyMedia variant=EmptyMediaVariant::Icon>
                                        <BlocksMark />
                                    </EmptyMedia>
                                    <EmptyTitle>"No blocks yet"</EmptyTitle>
                                    <EmptyDescription>
                                        "This is where compositions will go — a picker inside a
                                        popover, a cropper inside a dialog, a table that pages,
                                        a sidebar that is the whole frame of an app. The page
                                        exists for every component already; seven of them have
                                        examples so far."
                                    </EmptyDescription>
                                </EmptyHeader>
                            </Empty>
                        }
                            .into_any();
                    }
                    blocks
                        .iter()
                        .map(|block| {
                            view! {
                                <DemoSection
                                    title=block.title
                                    description=block.description
                                    code=block.code
                                >
                                    {(block.view)()}
                                </DemoSection>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
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
