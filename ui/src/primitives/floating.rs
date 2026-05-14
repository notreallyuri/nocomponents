use std::time::Duration;

use crate::utils::types::TriggerRect;
use leptos::{either::Either, ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Copy, Clone)]
pub struct FloatingContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,

    // Trigger
    pub trigger_ref: AnyNodeRef,
    pub trigger_rect: RwSignal<TriggerRect>,

    // For controlled usage (select, combobox)
    pub value: RwSignal<Option<String>>,
    pub display_value: RwSignal<Option<String>>,

    // Accessibility
    pub trigger_id: RwSignal<String>,
    pub content_id: RwSignal<String>,

    // Behavior flags
    pub modal: RwSignal<bool>,
    pub disabled: RwSignal<bool>,
}

impl Default for FloatingContext {
    fn default() -> Self {
        Self {
            is_open: RwSignal::new(false),
            is_mounted: RwSignal::new(false),
            trigger_ref: AnyNodeRef::new(),
            trigger_rect: RwSignal::new(TriggerRect::default()),
            value: RwSignal::new(None),
            display_value: RwSignal::new(None),
            trigger_id: RwSignal::new(String::new()),
            content_id: RwSignal::new(String::new()),
            modal: RwSignal::new(false),
            disabled: RwSignal::new(false),
        }
    }
}

impl FloatingContext {
    pub fn open(&self) {
        self.is_open.set(true)
    }

    pub fn close(&self) {
        self.is_open.set(false)
    }

    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open)
    }

    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }
}

#[component]
pub fn FloatingTrigger(
    context: FloatingContext,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<
        Callback<(AnyNodeRef, Callback<MouseEvent>), AnyView>,
    >,
    children: Children,
) -> impl IntoView {
    let on_click = Callback::new(move |_: MouseEvent| context.toggle());

    match render {
        Some(render_fn) => Either::Left(render_fn.run((context.trigger_ref, on_click))),
        None => Either::Right(view! {
            <button
                type="button"
                disabled=disabled
                class=class
                on:click=move |_| context.toggle()
                node_ref=context.trigger_ref
            >
                {children()}
            </button>
        }),
    }
}

#[component]
pub fn FloatingProvider(context: FloatingContext, children: Children) -> impl IntoView {
    provide_context(context);
    children()
}

#[component]
pub fn FloatingRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = 150, into)] unmount_delay: u64,
    children: Children,
) -> impl IntoView {
    let context = FloatingContext::default();
    provide_context(context);
    Effect::new(move |_| {
        if context.is_open.get() {
            context.is_mounted.set(true);
        } else {
            set_timeout(
                move || context.is_mounted.set(false),
                Duration::from_millis(unmount_delay),
            );
        }
    });
    view! { <div class=class>{children()}</div> }
}
