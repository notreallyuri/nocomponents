use crate::utils::{
    get_placement,
    types::{Align, Side, SideOffset},
};
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{either::Either, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct PopoverContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,
    pub trigger_ref: AnyNodeRef,
}

impl PopoverContext {
    pub fn close(&self) {
        self.is_open.set(false);
    }
    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open);
    }
}

pub fn use_popover() -> PopoverContext {
    expect_context::<PopoverContext>()
}

#[component]
pub fn PopoverRoot(
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(false));
    let is_mounted = RwSignal::new(is_open.get_untracked());
    let trigger_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if is_open.get() {
            is_mounted.set(true);
        } else {
            set_timeout(move || is_mounted.set(false), Duration::from_millis(150));
        }
    });

    provide_context(PopoverContext {
        is_open,
        is_mounted,
        trigger_ref,
    });

    view! { {children()} }
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
pub fn PopoverPortalRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_popover();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        context.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(get_placement(side, align))
            .strategy(Strategy::Fixed)
            .middleware(SendWrapper::new(middleware))
            .while_elements_mounted_auto_update(),
    );

    view! {
        <Show when=move || context.is_mounted.get()>
            <div class="fixed inset-0 z-40" on:click=move |_| context.close() />

            <div node_ref=floating_ref style=move || floating_styles.get() class="z-50 w-max">
                <div
                    data-state=move || if context.is_open.get() { "open" } else { "closed" }
                    data-slot="popover-portal"
                    data-side=match side {
                        Side::Top => "top",
                        Side::Bottom => "bottom",
                        Side::Left => "left",
                        Side::Right => "right",
                    }
                    class=class
                >
                    {children()}
                </div>
            </div>
        </Show>
    }
}
