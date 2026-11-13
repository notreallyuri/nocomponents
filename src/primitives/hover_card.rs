//! A card that opens on hover, whose contents you can actually reach.
//!
//! The tooltip's hover-intent shape with one difference: the content is interactive, so the
//! pointer must be able to leave the trigger and enter the card. Both cancel the pending close,
//! and the card keeps `pointer-events`, which a tooltip drops.

use crate::{
    primitives::floating::{
        FloatingContext, FloatingPortal, FloatingRoot, TriggerAria, floating_options,
        side_placement,
    },
    utils::{
        controllable::controllable,
        types::{Align, AnchorRender, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{context::Provider, either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;

pub type HoverCardContext = FloatingContext;

pub fn use_hover_card() -> HoverCardContext {
    expect_context::<HoverCardContext>()
}

/// The open/close intent shared by the trigger and the content.
#[derive(Clone, Copy)]
struct HoverIntent {
    context: HoverCardContext,
    open_delay: Signal<u64>,
    close_delay: Signal<u64>,
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
            self.open_delay.get_untracked()
        } else {
            self.close_delay.get_untracked()
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
    #[prop(into, default = 600u64.into())]
    delay: Signal<u64>,
    /// The grace period after leaving, long enough to cross the gap into the card.
    #[prop(into, default = 250u64.into())]
    close_delay: Signal<u64>,
    #[prop(optional)] context: Option<HoverCardContext>,
    /// Whether the card is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the card starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the card opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let mut context = context.unwrap_or_default();
    context.is_open = controllable(open, default_open, on_open_change);
    let stored_children = StoredValue::new(children);

    let intent = HoverIntent {
        context,
        open_delay: delay,
        close_delay,
        pending: StoredValue::new(None),
    };

    on_cleanup(move || intent.cancel());

    view! {
        <FloatingRoot class=class context=context trigger_aria=TriggerAria::Describes>
            <Provider value=intent>{stored_children.with_value(|children| children())}</Provider>
        </FloatingRoot>
    }
}

#[component]
pub fn HoverCardTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Render the trigger as something else — a link, a `Button`. Forward the node ref.
    #[prop(default = None, into)]
    render: Option<Callback<AnchorRender, AnyView>>,
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
                Some(render) => {
                    Either::Left(
                        render
                            .run(AnchorRender {
                                node_ref: ctx.trigger_ref,
                            }),
                    )
                }
                None => {
                    Either::Right(
                        view! {
                            <span node_ref=ctx
                                .trigger_ref>{children.map(|children| children())}</span>
                        },
                    )
                }
            }}
        </div>
    }
}

#[component]
pub fn HoverCardPortalRoot(
    /// Mount the layer in this element instead of `<body>` — a sticky or fixed ancestor it should
    /// move with. The fullscreen element still wins when the container is not inside it.
    #[prop(default = None)]
    container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_hover_card();
    let stored_children = StoredValue::new(children);

    view! {
        <FloatingPortal container=container>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn HoverCardContentRoot(
    #[prop(into, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(into, default = SideOffset(8.0).into())] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_hover_card();
    let intent = expect_context::<HoverIntent>();
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
