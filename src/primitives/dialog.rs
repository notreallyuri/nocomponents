use leptos::{either::Either, ev, portal::Portal, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct DialogContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,
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
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(false));
    let is_mounted = RwSignal::new(false);

    Effect::new(move |_| {
        if is_open.get() {
            is_mounted.set(true);
        } else {
            set_timeout(move || is_mounted.set(false), Duration::from_millis(100));
        }
    });

    let _ = window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && is_open.get() {
            is_open.set(false);
        }
    });

    provide_context(DialogContext {
        is_mounted,
        is_open,
    });

    view! { {children()} }
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
            <button type="button" class=class disabled=disabled on:click=move |_| ctx.toggle()>
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

    Effect::new(move |_| {
        let is_open = context.is_open.get();
        if let Some(body) = document().body() {
            if is_open {
                let _ = body.style().set_property("overflow", "hidden");
            } else {
                let _ = body.style().remove_property("overflow");
            }
        }
    });

    on_cleanup(move || {
        if let Some(body) = document().body() {
            let _ = body.style().remove_property("overflow");
        }
    });

    view! {
        <Portal>
            <Show when=move || context.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn DialogContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dialog();
    let content_ref = AnyNodeRef::new();

    let focus_trap = move |e: ev::KeyboardEvent| {
        if !ctx.is_open.get() {
            return;
        }

        if e.key() == "Tab" {
            if let Some(container) = content_ref.get() {
                let selector = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

                if let Ok(node_list) = container.query_selector_all(selector) {
                    let mut focusable = Vec::new();
                    for i in 0..node_list.length() {
                        if let Some(node) = node_list.item(i) {
                            if let Ok(el) = node.dyn_into::<HtmlElement>() {
                                focusable.push(el);
                            }
                        }
                    }

                    if focusable.is_empty() {
                        e.prevent_default();
                        return;
                    }

                    let first = focusable.first().unwrap();
                    let last = focusable.last().unwrap();
                    let active = document().active_element();

                    if e.shift_key() {
                        if active.as_ref() == Some(first.as_ref()) {
                            e.prevent_default();
                            let _ = last.focus();
                        }
                    } else {
                        if active.as_ref() == Some(last.as_ref()) {
                            e.prevent_default();
                            let _ = first.focus();
                        }
                    }
                }
            }
        }
    };

    let _ = window_event_listener(ev::keydown, focus_trap);

    Effect::new(move |_| {
        if ctx.is_open.get() {
            set_timeout(
                move || {
                    if let Some(container) = content_ref.get() {
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
                },
                Duration::from_millis(10),
            );
        }
    });

    view! {
        <div
            node_ref=content_ref
            class=class
            role="dialog"
            aria-modal="true"
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
}
