use leptos::{either::Either, ev, portal::Portal, prelude::*};
use std::time::Duration;

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

    view! {
        <Portal>
            <Show when=move || context.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}
