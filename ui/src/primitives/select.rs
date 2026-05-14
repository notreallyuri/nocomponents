use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift,
    ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[derive(Copy, Clone)]
pub struct SelectContext {
    pub is_open: RwSignal<bool>,
    pub value: RwSignal<Option<String>>,
    pub trigger_ref: AnyNodeRef,
}

impl SelectContext {
    pub fn close(&self) {
        self.is_open.set(false);
    }
    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open)
    }
}

pub fn use_select() -> SelectContext {
    expect_context::<SelectContext>()
}

#[component]
pub fn SelectRoot(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let trigger_ref = AnyNodeRef::new();

    provide_context(SelectContext {
        is_open,
        value,
        trigger_ref,
    });

    view! {
        <div class=class>
            {children()} // The hidden native select ensures form submissions still work!
            <select class="hidden" aria-hidden="true" tabindex="-1">
                <option value=move || value.get().unwrap_or_default() selected=true></option>
            </select>
        </div>
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
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match as_child {
        Some(render_fn) => Either::Left(render_fn.run((ctx.trigger_ref, on_click))),
        None => Either::Right(view! {
            <button
                type="button"
                disabled=disabled
                class=class
                on:click=move |_| ctx.toggle()
                node_ref=ctx.trigger_ref
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
pub fn SelectContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_select();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(0.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        x,
        y,
        strategy,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(Placement::BottomStart)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    view! {
        <Portal>
            <div
                class=move || if ctx.is_open.get() { "fixed inset-0 z-40" } else { "hidden" }
                on:click=move |_| ctx.close()
            ></div>

            <div
                node_ref=floating_ref
                data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    _ => "bottom",
                }
                style=move || {
                    if !ctx.is_open.get() {
                        return "display: none;".to_string();
                    }
                    let width = ctx
                        .trigger_ref
                        .get()
                        .map(|el| el.get_bounding_client_rect().width())
                        .unwrap_or(0.0);
                    format!(
                        "position: {}; left: {}px; top: {}px; width: {}px;",
                        match strategy.get() {
                            Strategy::Absolute => "absolute",
                            Strategy::Fixed => "fixed",
                        },
                        x.get(),
                        y.get(),
                        width,
                    )
                }
                class=class
            >
                {children()}
            </div>
        </Portal>
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
