//! An input and its trimmings, behaving as one control.
//!
//! Almost all of an input group is styling — the border moves off the `<input>` and onto the box
//! around it, so an icon or a button can sit inside what still looks like one field. The one thing
//! that is behaviour, and the reason this is a primitive at all: the box has padding and addons,
//! and clicking any of that chrome has to land in the control. A real `<input>` has no dead zone
//! inside its own border, and a group that does feels broken.
//!
//! Where an addon sits is published as `data-align`, so the styled layer can lay the group out
//! without any of this knowing what it looks like.

use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement};

/// Where an addon sits relative to the control. The inline pair share the control's line; the
/// block pair take a row of their own above or below it.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputGroupAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputGroupAlign::InlineStart => "inline-start",
            InputGroupAlign::InlineEnd => "inline-end",
            InputGroupAlign::BlockStart => "block-start",
            InputGroupAlign::BlockEnd => "block-end",
        }
    }
}

/// Anything that owns the click already — pressing a button in an addon must press the button, not
/// steal focus back to the input.
const INTERACTIVE: &str = "input, textarea, select, button, a, [role='button'], [contenteditable]";

#[component]
pub fn InputGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let node_ref = AnyNodeRef::new();

    // `mousedown` rather than `click`: focus has to be redirected before the browser hands it to
    // whatever was actually under the pointer, and `prevent_default` is what stops it going there.
    let focus_control = move |e: ev::MouseEvent| {
        let Some(node) = node_ref.get_untracked() else {
            return;
        };
        let Ok(group) = node.dyn_into::<Element>() else {
            return;
        };

        if let Some(target) = e.target()
            && let Ok(target) = target.dyn_into::<Element>()
            && matches!(target.closest(INTERACTIVE), Ok(Some(_)))
        {
            return;
        }

        if let Ok(Some(control)) = group.query_selector("input, textarea")
            && let Ok(control) = control.dyn_into::<HtmlElement>()
        {
            e.prevent_default();
            let _ = control.focus();
        }
    };

    view! {
        <div
            node_ref=node_ref
            role="group"
            data-slot="input-group"
            on:mousedown=focus_control
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupAddonRoot(
    #[prop(optional)] align: InputGroupAlign,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="input-group-addon" data-align=align.as_str() class=class>
            {children()}
        </div>
    }
}
