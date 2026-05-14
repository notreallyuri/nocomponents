use crate::{
    primitives::floating::{FloatingContext, FloatingRoot, FloatingTrigger},
    utils::{
        get_placement,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift,
    ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

pub type DropdownMenuContext = FloatingContext;

pub fn use_dropdown() -> DropdownMenuContext {
    expect_context::<DropdownMenuContext>()
}

#[component]
pub fn DropdownMenuRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <FloatingRoot class=class>{children()}</FloatingRoot> }
}

#[component]
pub fn DropdownMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dropdown();

    view! {
        <FloatingTrigger context=ctx class=class disabled=disabled render=render>
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn DropdownMenuPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_dropdown();
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
pub fn DropdownMenuContentRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
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
            .placement(get_placement(side, align))
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                }
            }
            class="fixed z-50 w-max"
        >
            <div
                data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
                data-align=move || match placement.get() {
                    Placement::TopStart
                    | Placement::BottomStart
                    | Placement::LeftStart
                    | Placement::RightStart => "start",
                    Placement::TopEnd
                    | Placement::BottomEnd
                    | Placement::LeftEnd
                    | Placement::RightEnd => "end",
                    _ => "center",
                }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => "bottom",
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
                    Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
                }
                class=class
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DropdownMenuItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dropdown();

    let handle_click = move |e: ev::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        ctx.close();
    };

    view! {
        <div on:click=handle_click class=class>
            {children()}
        </div>
    }
}
