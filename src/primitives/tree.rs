//! A tree: nested disclosure, one tab stop, and the arrow keys doing the walking.
//!
//! The item *is* the `treeitem`, and it contains both its own row and the group of its children —
//! which is what ARIA asks for (`tree` owns `treeitem`, `treeitem` owns `group`) and the reason the
//! row cannot simply be the item: a `role="none"` wrapper between the two would break the
//! ownership, and a sibling group would not be owned by anything.
//!
//! Containing its subtree does mean the item's accessible name would otherwise be every label
//! underneath it, so each item points `aria-labelledby` at its own row.
//!
//! A closed group renders nothing rather than hiding: an item nobody can see must not be an item
//! the arrow keys can reach, and [`crate::primitives::roving_focus`] finds its items in the DOM.
//! That is also what makes Down "the next *visible* item" without anything having to compute it.

use crate::{
    primitives::roving_focus::{RovingFocus, use_roving_focus},
    utils::{next_id, types::Orientation},
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;

#[derive(Copy, Clone)]
pub struct TreeContext {
    pub roving: RovingFocus,
}

/// What an item tells the items nested inside it.
#[derive(Copy, Clone)]
pub struct TreeItemContext {
    pub is_open: RwSignal<bool>,
    /// 1 at the root, as `aria-level` counts.
    pub level: usize,
    /// Set by [`TreeGroupRoot`] when there is one. A leaf must not announce `aria-expanded`, and
    /// whether an item has children is not known until its children have rendered.
    pub has_group: RwSignal<bool>,
}

impl TreeItemContext {
    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open);
    }
}

pub fn use_tree() -> TreeContext {
    expect_context::<TreeContext>()
}

/// The item this is called inside, or `None` at the top level.
pub fn use_tree_item() -> Option<TreeItemContext> {
    use_context::<TreeItemContext>()
}

#[component]
pub fn TreeRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Names the tree for assistive tech. A tree with no label is a list of rows nobody can place.
    #[prop(optional, into)]
    label: Signal<String>,
    children: Children,
) -> impl IntoView {
    let tree_ref = AnyNodeRef::new();
    let roving = use_roving_focus(tree_ref, Orientation::Vertical);
    let context = TreeContext { roving };

    // Without this every item is `tabindex="-1"` and the tree cannot be tabbed into at all. It
    // re-runs as branches open, and returns early once something already holds the stop.
    Effect::new(move |_| {
        if tree_ref.get().is_some() {
            roving.sync_tab_stop();
        }
    });

    view! {
        <div
            node_ref=tree_ref
            data-slot="tree"
            role="tree"
            aria-label=move || {
                let label = label.get();
                (!label.is_empty()).then_some(label)
            }
            on:keydown=move |e: ev::KeyboardEvent| {
                if roving.on_keydown(&e.key()) {
                    e.prevent_default();
                }
            }
            on:focusin=move |_| roving.on_focus_in()
            class=class
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
}

#[component]
pub fn TreeItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Open to begin with. Uncontrolled: the item owns its state from here on.
    #[prop(optional)]
    default_open: bool,
    children: Children,
) -> impl IntoView {
    let parent = use_tree_item();
    let context = TreeItemContext {
        is_open: RwSignal::new(default_open),
        level: parent.map(|p| p.level + 1).unwrap_or(1),
        has_group: RwSignal::new(false),
    };
    let row_id = StoredValue::new(next_id("tree-item"));
    let item_ref = AnyNodeRef::new();
    let tree = use_tree();

    // `aria-level` onto the node rather than into the markup: leptos has no attribute method for
    // it, and an `attr:` on a native element renders nothing at all — which is how this shipped
    // once and read as a tree with no depth.
    Effect::new(move |_| {
        if let Some(node) = item_ref.get() {
            let _ = node.set_attribute("aria-level", &context.level.to_string());
        }
    });

    view! {
        <div
            node_ref=item_ref
            data-slot="tree-item"
            role="treeitem"
            tabindex="-1"
            aria-labelledby=move || row_id.get_value()
            aria-expanded=move || {
                context.has_group.get().then(|| context.is_open.get().to_string())
            }
            data-state=move || if context.is_open.get() { "open" } else { "closed" }
            data-roving-item=""
            on:keydown=move |e: ev::KeyboardEvent| {
                let focused = item_ref
                    .get_untracked()
                    .is_some_and(|node| {
                        document().active_element().is_some_and(|active| active == node)
                    });
                if !focused {
                    return;
                }
                let handled = match e.key().as_str() {
                    "ArrowRight" if context.has_group.get_untracked() => {
                        if context.is_open.get_untracked() {
                            tree.roving.focus_offset(1);
                        } else {
                            context.is_open.set(true);
                        }
                        true
                    }
                    "ArrowLeft" => {
                        if context.has_group.get_untracked() && context.is_open.get_untracked() {
                            context.is_open.set(false);
                        } else {
                            tree.roving.focus_offset(-1);
                        }
                        true
                    }
                    "Enter" | " " => {
                        if let Some(row) = item_ref
                            .get_untracked()
                            .and_then(|node| node.query_selector("[data-slot=\"tree-row\"]").ok())
                            .flatten()
                            .and_then(|row| row.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            row.click();
                        }
                        true
                    }
                    _ => false,
                };
                if handled {
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            class=class
        >
            <Provider value=context>
                <Provider value=TreeRowId(row_id)>{children()}</Provider>
            </Provider>
        </div>
    }
}

/// The row's id, so the item can be named by it.
#[derive(Copy, Clone)]
struct TreeRowId(StoredValue<String>);

/// The part of an item you can see and click: its twisty, its icon, its label.
///
/// Not focusable — the item is, and the keys are handled there. This carries the id the item is
/// named by, and the click, which Enter and Space are routed through so there is one path rather
/// than two that have to be kept saying the same thing.
#[component]
pub fn TreeRowRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] on_select: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<TreeItemContext>();
    let row_id = expect_context::<TreeRowId>().0;

    view! {
        <div
            id=move || row_id.get_value()
            data-slot="tree-row"
            // The item's state, repeated on the row. Styling the row off the *item* means styling
            // off an ancestor, and a named group matches any ancestor that has it — so a closed
            // item inside an open one reads as open. Rows do not nest; the item's does.
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            on:click=move |e: ev::MouseEvent| {
                if ctx.has_group.get_untracked() {
                    ctx.toggle();
                }
                if let Some(on_select) = on_select {
                    on_select.run(e);
                }
            }
            class=class
        >
            {children()}
        </div>
    }
}

/// The children of an item. Its presence is what makes the item a branch.
#[component]
pub fn TreeGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<TreeItemContext>();
    ctx.has_group.set(true);

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.is_open.get()>
            <div data-slot="tree-group" role="group" class=class>
                {children.with_value(|c| c())}
            </div>
        </Show>
    }
}
