use crate::{
    cn,
    icons::chevron::ChevronRight,
    primitives::tree::{TreeGroupRoot, TreeItemRoot, TreeRoot, TreeRowRoot, use_tree_item},
};
use leptos::{ev, prelude::*};

#[component]
pub fn Tree(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] label: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <TreeRoot
            label=label
            class=move || cn!("flex w-full min-w-0 flex-col gap-0.5", class.get())
        >
            {children()}
        </TreeRoot>
    }
}

#[component]
pub fn TreeItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] default_open: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <TreeItemRoot
            default_open=default_open
            class=move || cn!("flex min-w-0 flex-col outline-none", class.get())
        >
            {children()}
        </TreeItemRoot>
    }
}

#[component]
pub fn TreeRow(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(default = None, into)] on_select: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let item = use_tree_item();
    let level = item.map(|item| item.level).unwrap_or(1);
    let has_group = move || item.is_some_and(|item| item.has_group.get());

    view! {
        <TreeRowRoot
            on_select=on_select
            class=move || {
                cn!(
                    "group/tree-row flex h-8 min-w-0 cursor-default items-center gap-1.5 rounded-md pr-2 text-sm transition-colors",
                    "hover:bg-accent hover:text-accent-foreground",
                    // The item is what takes focus, and the row is its direct child — a `group-*`
                    // would ring every row under a focused branch, since it matches any ancestor.
                    "[[data-slot=tree-item]:focus-visible>&]:ring-2 [[data-slot=tree-item]:focus-visible>&]:ring-ring",
                    "data-[active=true]:bg-accent data-[active=true]:font-medium data-[active=true]:text-accent-foreground",
                    class.get(),
                )
            }
            attr:data-active=move || active.get().then_some("true")
            attr:style=move || format!("padding-left: {}rem", 0.375 + (level - 1) as f64 * 0.75)
        >
            <span class="flex size-4 shrink-0 items-center justify-center">
                <Show when=has_group>
                    <ChevronRight class="size-3.5 text-muted-foreground transition-transform group-data-[state=open]/tree-row:rotate-90" />
                </Show>
            </span>
            <span class="truncate">{children()}</span>
        </TreeRowRoot>
    }
}

#[component]
pub fn TreeGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <TreeGroupRoot class=move || {
            cn!("flex min-w-0 flex-col gap-0.5", class.get())
        }>{children()}</TreeGroupRoot>
    }
}
