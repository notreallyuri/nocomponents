//! A panel that opens and closes.
//!
//! The only hard part is animating to a height nobody wrote down. The content is measured after it
//! mounts and its own height is published as `--nc-content-height` on the element, which is what
//! the `collapsible-*` and `accordion-*` keyframes animate to — one variable, since both pairs
//! animate the same thing. Closing keeps the
//! content mounted for `unmount_delay` so the exit animation has something to run on, the same
//! trick the floating layers use.

use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

use crate::utils::next_id;

#[derive(Copy, Clone)]
pub struct CollapsibleContext {
    pub is_open: RwSignal<bool>,
    /// Stays true through the closing animation, so the content can play it before it goes.
    pub is_mounted: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub content_id: RwSignal<String>,
    pub content_ref: AnyNodeRef,
}

impl CollapsibleContext {
    pub fn toggle(&self) {
        if !self.disabled.get_untracked() {
            self.is_open.update(|open| *open = !*open);
        }
    }

    pub fn open(&self) {
        self.is_open.set(true);
    }

    pub fn close(&self) {
        self.is_open.set(false);
    }

    pub fn state(&self) -> &'static str {
        if self.is_open.get() { "open" } else { "closed" }
    }
}

pub fn use_collapsible() -> CollapsibleContext {
    expect_context::<CollapsibleContext>()
}

/// Publishes the content's natural height so the keyframes have something to animate to.
///
/// Reads the node ref *reactively*, so it also runs on the pass where the element first attaches —
/// the content mounts a tick after `is_open` flips, and measuring before that would silently do
/// nothing. Re-measures on every transition too: the content can change between openings, and a
/// stale height either clips the panel or leaves a gap at the end of the animation.
pub fn publish_content_height(content_ref: AnyNodeRef) {
    if let Some(el) = content_ref.get()
        && let Ok(el) = el.dyn_into::<HtmlElement>()
    {
        let height = el.scroll_height();
        let _ = el
            .style()
            .set_property("--nc-content-height", &format!("{height}px"));
    }
}

#[component]
pub fn CollapsibleRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Omit for an uncontrolled panel that owns its own state.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    #[prop(default = 200, into)] unmount_delay: u64,
    #[prop(optional)] context: Option<CollapsibleContext>,
    children: Children,
) -> impl IntoView {
    let context = context.unwrap_or_else(|| CollapsibleContext {
        is_open: open.unwrap_or_else(|| RwSignal::new(false)),
        is_mounted: RwSignal::new(open.map(|o| o.get_untracked()).unwrap_or(false)),
        disabled,
        content_id: RwSignal::new(next_id("collapsible-content")),
        content_ref: AnyNodeRef::new(),
    });

    let unmount_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cancel_unmount = move || {
        if let Some(handle) = unmount_handle.get_value() {
            handle.clear();
            unmount_handle.set_value(None);
        }
    };

    Effect::new(move |_| {
        if context.is_open.get() {
            cancel_unmount();
            context.is_mounted.set(true);
        } else if let Ok(handle) = set_timeout_with_handle(
            move || context.is_mounted.set(false),
            Duration::from_millis(unmount_delay),
        ) {
            unmount_handle.set_value(Some(handle));
        }
    });

    on_cleanup(cancel_unmount);

    provide_context(context);

    view! {
        <div data-slot="collapsible" data-state=move || context.state() class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_collapsible();
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    view! {
        <button
            type="button"
            data-slot="collapsible-trigger"
            aria-expanded=move || ctx.is_open.get().to_string()
            aria-controls=move || ctx.content_id.get()
            data-state=move || ctx.state()
            disabled=is_disabled
            on:click=move |_: ev::MouseEvent| ctx.toggle()
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_collapsible();
    let stored_children = StoredValue::new(children);

    // Measure on every transition, before the animation starts.
    Effect::new(move |_| {
        ctx.is_open.track();
        publish_content_height(ctx.content_ref);
    });

    view! {
        <Show when=move || ctx.is_mounted.get()>
            <div
                node_ref=ctx.content_ref
                id=move || ctx.content_id.get()
                data-slot="collapsible-content"
                data-state=move || ctx.state()
                class=class
            >
                {stored_children.with_value(|children| children())}
            </div>
        </Show>
    }
}
