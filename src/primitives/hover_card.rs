//! A card that opens on hover, whose contents you can actually reach.
//!
//! The tooltip's hover-intent shape with one difference: the content is interactive, so the
//! pointer must be able to leave the trigger and enter the card. Both cancel the pending close,
//! and the card keeps `pointer-events`, which a tooltip drops.

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
use leptos::{context::Provider, either::Either, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::time::Duration;

pub type HoverCardContext = FloatingContext;

pub fn use_hover_card() -> HoverCardContext {
    expect_context::<HoverCardContext>()
}

/// The open/close intent shared by the trigger and the content.
#[derive(Clone, Copy)]
struct HoverIntent {
    context: HoverCardContext,
    open_delay: u64,
    close_delay: u64,
    pending: StoredValue<Option<TimeoutHandle>>,
}

impl HoverIntent {
    fn cancel(&self) {
        if let Some(handle) = self.pending.get_value() {
            handle.clear();
            self.pending.set_value(None);
        }
    }

    fn schedule(&self, open: bool) {
        self.cancel();
        let context = self.context;
        let delay = if open {
            self.open_delay
        } else {
            self.close_delay
        };

        if let Ok(handle) = set_timeout_with_handle(
            move || context.is_open.set(open),
            Duration::from_millis(delay),
        ) {
            self.pending.set_value(Some(handle));
        }
    }
}

#[component]
pub fn HoverCardRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// How long the pointer has to rest before the card opens.
    #[prop(default = 600, into)]
    delay: u64,
    /// The grace period after leaving, long enough to cross the gap into the card.
    #[prop(default = 250, into)]
    close_delay: u64,
    #[prop(optional)] context: Option<HoverCardContext>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = context.unwrap_or_default();
    let stored_children = StoredValue::new(children);

    let intent = HoverIntent {
        context,
        open_delay: delay,
        close_delay,
        pending: StoredValue::new(None),
    };

    on_cleanup(move || intent.cancel());

    view! {
        <FloatingRoot class=class context=context trigger_aria=TriggerAria::Popup("dialog")>
            <Provider value=intent>{stored_children.with_value(|children| children())}</Provider>
        </FloatingRoot>
    }
}

#[component]
pub fn HoverCardTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Render the trigger as something else — a link, a `Button`. Forward the node ref.
    #[prop(default = None, into)]
    render: Option<Callback<AnyNodeRef, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_hover_card();
    let intent = expect_context::<HoverIntent>();

    view! {
        <div
            data-slot="hover-card-trigger"
            class=class
            on:pointerenter=move |_| intent.schedule(true)
            on:pointerleave=move |_| intent.schedule(false)
            on:focusin=move |_| intent.schedule(true)
            on:focusout=move |_| intent.schedule(false)
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
pub fn HoverCardPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_hover_card();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn HoverCardContentRoot(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(default = SideOffset(8.0))] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_hover_card();
    let intent = expect_context::<HoverIntent>();
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
            // Entering the card is not leaving the trigger: cancel its pending close.
            on:pointerenter=move |_| intent.cancel()
            on:pointerleave=move |_| intent.schedule(false)
        >
            <div
                id=move || ctx.content_id.get()
                data-slot="hover-card-content"
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
