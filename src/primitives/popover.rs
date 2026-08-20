use crate::{
    primitives::floating::{FloatingContext, FloatingRoot, FloatingTrigger, TriggerAria},
    utils::{
        get_placement,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

pub type PopoverContext = FloatingContext;

pub fn use_popover() -> PopoverContext {
    expect_context::<PopoverContext>()
}

#[component]
pub fn PopoverRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <FloatingRoot class=class trigger_aria=TriggerAria::Popup("dialog")>{children()}</FloatingRoot> }
}

#[component]
pub fn PopoverTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_popover();
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
pub fn PopoverPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_popover();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>
                {stored_children.with_value(|c| c())}
            </Show>
        </Portal>
    }
}

#[component]
pub fn PopoverContentRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_popover();
    let floating_ref = ctx.content_ref;

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
        // OUTER DIV: The Structural Anchor (Hidden on frame 1)
        <div
            node_ref=floating_ref
            style=move || {
                if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                }
            }
            class="fixed z-50 w-max pointer-events-none"
        >
            <div
                id=move || ctx.content_id.get()
                role="dialog"
                aria-labelledby=move || ctx.trigger_id.get()
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
                data-slot="popover-portal"
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
