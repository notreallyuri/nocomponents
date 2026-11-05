use crate::{
    primitives::floating::{
        FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria, TriggerRender,
        floating_options, side_placement,
    },
    utils::{
        controllable::forward,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{either::Either, prelude::*};

pub type PopoverContext = FloatingContext;

pub fn use_popover() -> PopoverContext {
    expect_context::<PopoverContext>()
}

#[component]
pub fn PopoverRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether the popover is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the popover starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the popover opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <FloatingRoot
            class=class
            trigger_aria=TriggerAria::Popup("dialog")
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
        >
            {children()}
        </FloatingRoot>
    }
}

#[component]
pub fn PopoverTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_popover();
    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=disabled
            data_slot="popover-trigger"
            render=as_child
        >
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
        <FloatingPortal>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn PopoverContentRoot(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_popover();
    let floating_ref = ctx.content_ref;

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options(side_placement(side, align), side_offset),
    );

    view! {
        // The positioned shell, hidden until floating-ui has measured it.
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
                data-slot="popover-content"
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
