use crate::primitives::floating::{FloatingContext, FloatingRoot, FloatingTrigger};
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift,
    ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

pub type SelectContext = FloatingContext;

pub fn use_select() -> SelectContext {
    expect_context::<SelectContext>()
}
#[component]
pub fn SelectRoot(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = FloatingContext {
        value,
        ..Default::default()
    };

    view! {
        <FloatingRoot class=class context=ctx>
            {children()}
            <select class="hidden" aria-hidden="true" tabindex="-1">
                <option value=move || ctx.value.get().unwrap_or_default() selected=true></option>
            </select>
        </FloatingRoot>
    }
}

#[component]
pub fn SelectTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_select();
    view! {
        <FloatingTrigger context=ctx class=class disabled=disabled render=as_child>
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn SelectPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_select();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>
                <div
                    class=move || if ctx.is_open.get() { "fixed inset-0 z-40" } else { "hidden" }
                    on:click=move |_| ctx.close()
                ></div>

                {stored_children.with_value(|c| c())}
            </Show>
        </Portal>
    }
}

#[component]
pub fn SelectContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(0.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(Placement::BottomStart)
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                let width = ctx
                    .trigger_ref
                    .get()
                    .map(|el| el.get_bounding_client_rect().width())
                    .unwrap_or(0.0);
                let mut base_style = if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                };
                base_style.push_str(&format!(" width: {}px;", width));
                base_style
            }
            class="fixed z-50"
        >
            <div
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    _ => "bottom",
                }
                class=class
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn SelectItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let item_value = value.clone();

    let is_selected = move || ctx.value.get() == Some(item_value.clone());

    let on_click = move |_: ev::MouseEvent| {
        ctx.value.set(Some(value.clone()));
        ctx.close();
    };

    view! {
        <div
            on:click=on_click
            data-state=move || if is_selected() { "checked" } else { "unchecked" }
            class=class
        >
            {children()}
        </div>
    }
}
