use crate::{
    primitives::floating::{
        FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria, TriggerRender,
        floating_options, shell_class, side_placement,
    },
    utils::{
        controllable::forward,
        sheet::{SheetHeader, SheetScrim, presentation_attr, use_sheet_presentation},
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;

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
pub fn PopoverPortalRoot(
    /// Mount the popover in this element instead of `<body>` — a sticky or fixed ancestor it
    /// should move with, so scrolling does not leave it behind until the next position update.
    /// The fullscreen element still wins when the container is not inside it.
    #[prop(default = None)]
    container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_popover();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingPortal container=container>
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
    /// Below the `md` breakpoint, present the popover as a bottom sheet: a scrim, the panel on
    /// the bottom edge, no positioning. Written as `data-presentation="sheet"` (`"floating"`
    /// otherwise) for the styled layer to key on. Off by default.
    #[prop(optional, into)]
    mobile_sheet: Signal<bool>,
    /// The title row shown at the top of the sheet. Empty means a grabber and no row.
    #[prop(optional, into)]
    sheet_title: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_popover();
    let floating_ref = ctx.content_ref;
    let sheet = use_sheet_presentation(mobile_sheet);

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

    let ready = Signal::derive(move || sheet.get() || is_positioned.get());

    view! {
        <SheetScrim sheet=sheet open=ctx.is_open.into() />
        // The positioned shell, hidden until floating-ui has measured it.
        <div
            node_ref=floating_ref
            style=move || {
                if sheet.get() {
                    String::new()
                } else if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                }
            }
            class=move || shell_class(sheet.get(), "fixed z-50 w-max pointer-events-none")
        >
            <div
                id=move || ctx.content_id.get()
                role="dialog"
                aria-labelledby=move || ctx.trigger_id.get()
                data-state=move || {
                    if ctx.is_open.get() && ready.get() { "open" } else { "closed" }
                }
                data-slot="popover-content"
                data-presentation=move || presentation_attr(sheet.get())
                data-align=move || (!sheet.get()).then(|| match placement.get() {
                    Placement::TopStart
                    | Placement::BottomStart
                    | Placement::LeftStart
                    | Placement::RightStart => "start",
                    Placement::TopEnd
                    | Placement::BottomEnd
                    | Placement::LeftEnd
                    | Placement::RightEnd => "end",
                    _ => "center",
                })
                data-side=move || (!sheet.get()).then(|| match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => "bottom",
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
                    Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
                })
                class=class
            >
                <SheetHeader sheet=sheet title=sheet_title />
                {children()}
            </div>
        </div>
    }
}
