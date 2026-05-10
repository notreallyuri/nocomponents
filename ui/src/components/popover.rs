use super::{
    button::{Button, ButtonSize, ButtonVariant},
    common::{Align, Side, SideOffset},
};
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift,
    ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[derive(Copy, Clone)]
struct PopoverContext {
    is_open: RwSignal<bool>,
    trigger_ref: AnyNodeRef,
}

fn get_placement(side: Side, align: Align) -> Placement {
    match (side, align) {
        (Side::Top, Align::Start) => Placement::TopStart,
        (Side::Top, Align::Center) => Placement::Top,
        (Side::Top, Align::End) => Placement::TopEnd,
        (Side::Bottom, Align::Start) => Placement::BottomStart,
        (Side::Bottom, Align::Center) => Placement::Bottom,
        (Side::Bottom, Align::End) => Placement::BottomEnd,
        (Side::Left, Align::Start) => Placement::LeftStart,
        (Side::Left, Align::Center) => Placement::Left,
        (Side::Left, Align::End) => Placement::LeftEnd,
        (Side::Right, Align::Start) => Placement::RightStart,
        (Side::Right, Align::Center) => Placement::Right,
        (Side::Right, Align::End) => Placement::RightEnd,
    }
}

#[component]
pub fn Popover(children: Children) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let trigger_ref = AnyNodeRef::new();

    provide_context(PopoverContext {
        is_open,
        trigger_ref,
    });

    view! { <div class="w-fit h-fit">{children()}</div> }
}

#[component]
pub fn PopoverTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = expect_context::<PopoverContext>();

    view! {
        <div node_ref=context.trigger_ref class="inline-flex w-fit h-fit">
            <Button
                size=size
                variant=variant
                class=class
                disabled=disabled
                on_click=move |_| context.is_open.update(|open| *open = !*open)
            >
                {match children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(""),
                }}
            </Button>
        </div>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<PopoverContext>();
    let floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0 as f64))),
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

    let base_classes = "flex flex-col gap-2.5 rounded-lg bg-popover p-2.5 text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10 outline-none duration-100 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95";

    view! {
        <Show when=move || context.is_open.get()>
            <div class="fixed inset-0 z-40" on:click=move |_| context.is_open.set(false) />

            <div node_ref=floating_ref style=move || floating_styles.get() class="z-50 w-max">
                <div
                    data-state="open"
                    data-side=match side {
                        Side::Top => "top",
                        Side::Bottom => "bottom",
                        Side::Left => "left",
                        Side::Right => "right",
                    }
                    class=move || {
                        let extra = class.get();
                        format!("{base_classes} {extra}")
                    }
                >
                    {children()}
                </div>
            </div>
        </Show>
    }
}
