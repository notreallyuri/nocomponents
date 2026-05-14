use crate::{
    primitives::{
        floating::FloatingContext,
        middleware::{TransformOrigin, TransformOriginData},
    },
    utils::{
        get_placement,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, LimitShift, LimitShiftOptions, MiddlewareVec, Offset,
    OffsetOptions, Placement, Shift, ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
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
    children: Children,
) -> impl IntoView {
    provide_context(DropdownMenuContext::default());
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn DropdownMenuTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dropdown();
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
pub fn DropdownMenuContentRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_dropdown();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
        Box::new(Shift::new(
            ShiftOptions::default()
                .main_axis(true)
                .cross_axis(false)
                .limiter(Box::new(LimitShift::new(LimitShiftOptions::default()))),
        )),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(TransformOrigin),
    ];

    let UseFloatingReturn {
        x,
        y,
        strategy,
        placement,
        is_positioned,
        middleware_data,
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
        <Portal>
            <div
                class=move || if ctx.is_open.get() { "fixed inset-0 z-40" } else { "hidden" }
                on:click=move |_| ctx.close()
            ></div>

            <div
                node_ref=floating_ref
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
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

                style=move || {
                    if !ctx.is_open.get() {
                        return "display: none;".to_string();
                    }
                    let data = middleware_data.get();
                    let origin = data
                        .get_as::<TransformOriginData>("transformOrigin")
                        .map(|o| format!("{} {}", o.x, o.y))
                        .unwrap_or_else(|| "center center".to_string());
                    let mut base_style = format!(
                        "position: {}; left: {}px; top: {}px; --dropdown-menu-content-transform-origin: {};",
                        match strategy.get() {
                            Strategy::Absolute => "absolute",
                            Strategy::Fixed => "fixed",
                        },
                        x.get(),
                        y.get(),
                        origin,
                    );
                    if !is_positioned.get() {
                        base_style.push_str(" visibility: hidden;");
                    }
                    base_style
                }
                class=class
            >
                {children()}
            </div>
        </Portal>
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
