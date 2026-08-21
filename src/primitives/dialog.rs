use crate::{
    primitives::dismiss::{LayerBounds, use_dismissable_layer},
    utils::next_id,
};
use leptos::{
    context::Provider, either::Either, ev, portal::Portal, prelude::*, wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct DialogContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,

    /// Whether this dialog takes the page. A modal one traps focus, announces `aria-modal`, and is
    /// only dismissed through its own overlay; a non-modal one leaves the page usable underneath
    /// and closes when a pointer lands outside it.
    pub modal: bool,

    /// The panel and the button that opened it. Only a non-modal dialog needs them — a modal one
    /// is dismissed through its overlay — but they are minted here so the root can hand them to
    /// the dismissable layer, which is set up long before the content mounts.
    pub content_ref: AnyNodeRef,
    pub trigger_ref: AnyNodeRef,

    /// Ids of the title and description, empty until those parts render. The content points at
    /// whichever of them exist, so a dialog without a description does not claim to have one.
    pub title_id: RwSignal<String>,
    pub description_id: RwSignal<String>,
}

impl DialogContext {
    pub fn close(&self) {
        self.is_open.set(false);
    }
    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open);
    }
}

pub fn use_dialog() -> DialogContext {
    expect_context::<DialogContext>()
}

#[component]
pub fn DialogRoot(
    #[prop(default = 150, into)] unmount_delay: u64,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    /// Non-modal leaves the page interactive underneath. Defaults to modal, which is what a
    /// dialog, a sheet and an alert dialog all want.
    #[prop(default = true)]
    modal: bool,
    children: Children,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(false));
    let is_mounted = RwSignal::new(is_open.get_untracked());

    let context = DialogContext {
        modal,
        content_ref: AnyNodeRef::new(),
        trigger_ref: AnyNodeRef::new(),
        is_open,
        is_mounted,
        title_id: RwSignal::new(String::new()),
        description_id: RwSignal::new(String::new()),
    };

    // What had focus before the dialog took it, so it can be handed back on close.
    let previously_focused = StoredValue::new_local(None::<HtmlElement>);

    // The pending unmount, cancelled both when the layer reopens inside the delay and when the
    // root is disposed: a submenu is torn down together with the menu that contains it, and a
    // timer that fired after that would touch signals which no longer exist.
    let unmount_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    let cancel_unmount = move || {
        if let Some(handle) = unmount_handle.get_value() {
            handle.clear();
            unmount_handle.set_value(None);
        }
    };

    // `is_mounted` trails `is_open` by `unmount_delay` so the content survives long enough to
    // play its exit animation; `DialogPortalRoot` gates on it rather than on `is_open`.
    Effect::new(move |was_open: Option<bool>| {
        let is_open = context.is_open.get();

        if is_open {
            previously_focused.set_value(
                document()
                    .active_element()
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()),
            );
            cancel_unmount();
            context.is_mounted.set(true);
        } else {
            // Only on an actual close — not on the first run, when nothing was ever focused here.
            if was_open == Some(true)
                && let Some(el) = previously_focused.get_value()
            {
                let _ = el.focus();
            }
            if let Ok(handle) = set_timeout_with_handle(
                move || context.is_mounted.set(false),
                Duration::from_millis(unmount_delay),
            ) {
                unmount_handle.set_value(Some(handle));
            }
        }

        is_open
    });

    on_cleanup(cancel_unmount);

    // A modal layer swallows outside pointerdowns and is dismissed through its own overlay; a
    // non-modal one has no overlay to click, so the bounds are what tell it apart from outside.
    let bounds = if modal {
        LayerBounds::modal()
    } else {
        LayerBounds::new(context.content_ref, context.trigger_ref)
    };

    use_dismissable_layer(context.is_open, bounds, move || context.close());

    // Must be a `Provider` rather than a bare `provide_context`: `DialogPortalRoot` mounts its
    // children through leptos' `Portal`, which renders under an owner of its own. A context
    // provided on this component's owner is invisible from there, so `use_dialog()` inside the
    // portal resolves to some *other* dialog's context (the last one provided on the page) and
    // the content renders with a stale `data-state`. `FloatingRoot` scopes its context the same way.
    view! { <Provider value=context>{children()}</Provider> }
}

/// Registers an id on the context so `DialogContentRoot` can point `aria-labelledby` /
/// `aria-describedby` at this element, and returns it for the element to render.
pub fn use_dialog_label_id(part: DialogPart) -> String {
    let ctx = use_dialog();
    let id = next_id(match part {
        DialogPart::Title => "dialog-title",
        DialogPart::Description => "dialog-description",
    });

    match part {
        DialogPart::Title => ctx.title_id.set(id.clone()),
        DialogPart::Description => ctx.description_id.set(id.clone()),
    }

    id
}

/// The labelling parts of a dialog, for [`use_dialog_label_id`].
#[derive(Clone, Copy)]
pub enum DialogPart {
    Title,
    Description,
}

#[component]
pub fn DialogTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] as_child: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dialog();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match as_child {
        Some(render_fn) => Either::Left(render_fn.run(on_click)),
        None => Either::Right(view! {
            <button
                type="button"
                node_ref=ctx.trigger_ref
                class=class
                disabled=disabled
                on:click=move |_| ctx.toggle()
            >
                {match children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(""),
                }}
            </button>
        }),
    }
}

#[component]
pub fn DialogPortalRoot(children: ChildrenFn) -> impl IntoView {
    let context = use_dialog();
    let stored_children = StoredValue::new(children);

    // Only a modal dialog freezes the page behind it. A non-modal one exists precisely so the
    // page stays usable, and locking its scroll would take that back.
    Effect::new(move |_| {
        let is_open = context.is_open.get();
        if context.modal
            && let Some(body) = document().body()
        {
            if is_open {
                let _ = body.style().set_property("overflow", "hidden");
            } else {
                let _ = body.style().remove_property("overflow");
            }
        }
    });

    on_cleanup(move || {
        if context.modal
            && let Some(body) = document().body()
        {
            let _ = body.style().remove_property("overflow");
        }
    });

    view! {
        <Portal>
            <Show when=move || {
                context.is_mounted.get()
            }>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn DialogContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Inline geometry, for a caller that positions or transforms the panel itself — the drawer
    /// drags it, so its offset cannot live in a class.
    #[prop(optional, into)]
    style: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dialog();
    let content_ref = ctx.content_ref;

    let focus_trap = move |e: ev::KeyboardEvent| {
        // A non-modal dialog must not hold Tab: the page underneath is still the user's.
        if !ctx.is_open.get() || !ctx.modal {
            return;
        }

        if e.key() == "Tab"
            && let Some(container) = content_ref.get()
        {
            let selector = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

            if let Ok(node_list) = container.query_selector_all(selector) {
                let mut focusable = Vec::new();
                for i in 0..node_list.length() {
                    if let Some(node) = node_list.item(i)
                        && let Ok(el) = node.dyn_into::<HtmlElement>()
                    {
                        focusable.push(el);
                    }
                }

                if focusable.is_empty() {
                    e.prevent_default();
                    return;
                }

                let first = focusable.first().unwrap();
                let last = focusable.last().unwrap();
                let active = document().active_element();

                if e.shift_key() && active.as_ref() == Some(first.as_ref()) {
                    e.prevent_default();
                    let _ = last.focus();
                } else if active.as_ref() == Some(last.as_ref()) {
                    e.prevent_default();
                    let _ = first.focus();
                }
            }
        }
    };

    let _ = window_event_listener(ev::keydown, focus_trap);

    Effect::new(move |_| {
        if ctx.is_open.get()
            && let Some(container) = content_ref.get()
        {
            let selector = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

            if let Ok(Some(first_node)) = container.query_selector(selector) {
                if let Ok(el) = first_node.dyn_into::<HtmlElement>() {
                    let _ = el.focus();
                }
            } else {
                let _ = container.set_attribute("tabindex", "-1");
                if let Ok(el) = container.dyn_into::<HtmlElement>() {
                    let _ = el.focus();
                }
            }
        }
    });

    view! {
        <div
            node_ref=content_ref
            class=class
            style=style
            role="dialog"
            aria-modal=move || ctx.modal.then_some("true")
            aria-labelledby=move || Some(ctx.title_id.get()).filter(|id| !id.is_empty())
            aria-describedby=move || Some(ctx.description_id.get()).filter(|id| !id.is_empty())
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
}
