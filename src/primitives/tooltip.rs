//! A label that appears on hover or focus.
//!
//! Shares the floating layer with popover and friends, but not its trigger: a tooltip is not opened
//! by a click and does not *control* anything, so its trigger announces `aria-describedby` rather
//! than `aria-expanded` (see [`TriggerAria`]).
//!
//! The delays are the whole design. Opening waits `delay` so that sweeping the pointer across a
//! toolbar does not flash a row of tooltips; closing waits `close_delay`, a much shorter one, so
//! that crossing a gap between the trigger and its content does not dismiss it.

use crate::{
    primitives::floating::{FloatingContext, FloatingRoot, TriggerAria},
    utils::{
        get_placement,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{context::Provider, either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::time::Duration;

pub type TooltipContext = FloatingContext;

pub fn use_tooltip() -> TooltipContext {
    expect_context::<TooltipContext>()
}

/// The open and close delays, in milliseconds, shared with the trigger through context.
#[derive(Clone, Copy)]
struct TooltipDelays {
    open: u64,
    close: u64,
}

#[component]
pub fn TooltipRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// How long the pointer has to rest on the trigger before the tooltip opens.
    #[prop(default = 400, into)]
    delay: u64,
    /// The grace period after leaving, so a gap between trigger and content is survivable.
    #[prop(default = 100, into)]
    close_delay: u64,
    #[prop(optional)] context: Option<TooltipContext>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = context.unwrap_or_default();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingRoot class=class context=context trigger_aria=TriggerAria::Describes>
            <Provider value=TooltipDelays {
                open: delay,
                close: close_delay,
            }>{stored_children.with_value(|children| children())}</Provider>
        </FloatingRoot>
    }
}

#[component]
pub fn TooltipTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Render the trigger as something else — usually a `Button`. The node ref must be forwarded,
    /// or the tooltip has nothing to anchor to.
    #[prop(default = None, into)]
    render: Option<Callback<AnyNodeRef, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_tooltip();
    let delays = expect_context::<TooltipDelays>();

    let pending: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cancel = move || {
        if let Some(handle) = pending.get_value() {
            handle.clear();
            pending.set_value(None);
        }
    };

    let schedule = move |open: bool| {
        cancel();
        let delay = if open { delays.open } else { delays.close };
        if let Ok(handle) =
            set_timeout_with_handle(move || ctx.is_open.set(open), Duration::from_millis(delay))
        {
            pending.set_value(Some(handle));
        }
    };

    on_cleanup(cancel);

    let show = move || {
        if !disabled.get_untracked() {
            schedule(true);
        }
    };
    let hide = move || schedule(false);

    view! {
        <div
            data-slot="tooltip-trigger"
            class=class
            on:pointerenter=move |_| show()
            on:pointerleave=move |_| hide()
            // Focus counts as hovering: a tooltip that only answers to a pointer is invisible to
            // anyone driving the page from the keyboard.
            on:focusin=move |_| show()
            on:focusout=move |_| hide()
            // Pressing the trigger dismisses it — the tooltip has done its job by then, and it
            // would otherwise sit over whatever the click opened.
            on:click=move |_: ev::MouseEvent| {
                cancel();
                ctx.is_open.set(false);
            }
        >
            {match render {
                Some(render) => Either::Left(render.run(ctx.trigger_ref)),
                None => {
                    Either::Right(
                        view! {
                            <span node_ref=ctx.trigger_ref>
                                {children.map(|children| children())}
                            </span>
                        },
                    )
                }
            }}
        </div>
    }
}

#[component]
pub fn TooltipPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_tooltip();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn TooltipContentRoot(
    #[prop(default = Side::Top)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(default = SideOffset(6.0))] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_tooltip();
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
                role="tooltip"
                data-slot="tooltip-content"
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
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
