use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::popover::{PopoverContentRoot, PopoverPortalRoot, PopoverRoot, PopoverTriggerRoot},
    utils::controllable::forward,
};
use leptos::{either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[allow(unused_imports)]
pub use crate::utils::types::{Align, Side, SideOffset, TriggerRender};

#[component]
pub fn Popover(
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopoverRoot open=open default_open=default_open on_open_change=forward(on_open_change)>
            {children()}
        </PopoverRoot>
    }
}

#[component]
pub fn PopoverTrigger(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let styled = Callback::new(move |trigger: TriggerRender| {
        view! {
            <Button
                size=size
                variant=variant
                node_ref=trigger.node_ref
                on_click=trigger.on_click
                disabled=trigger.disabled
                class=class
            >
                {move || {
                    children
                        .with_value(|children| match children {
                            Some(child) => Either::Left(child()),
                            None => Either::Right(view! { "" }.into_any()),
                        })
                }}
            </Button>
        }
        .into_any()
    });

    view! { <PopoverTriggerRoot disabled=disabled as_child=as_child.unwrap_or(styled) /> }
}

#[component]
pub fn PopoverPortal(
    #[prop(default = None)] container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <PopoverPortalRoot container=container children=children /> }
}

#[component]
pub fn PopoverContent(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] mobile_sheet: Signal<bool>,
    #[prop(optional, into)] sheet_title: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <PopoverContentRoot
            side=side
            align=align
            side_offset=side_offset
            mobile_sheet=mobile_sheet
            sheet_title=sheet_title
            class=move || {
                let base_one = "flex flex-col gap-2 py-4 rounded-lg bg-popover text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10 outline-none animation-duration-100 fill-mode-forwards pointer-events-auto data-[state=closed]:pointer-events-none";
                let animations = "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
                let base_two = "has-data-[slot=popover-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl";
                let sheet = "data-[presentation=sheet]:w-full data-[presentation=sheet]:max-h-[85dvh] data-[presentation=sheet]:overflow-y-auto data-[presentation=sheet]:rounded-b-none data-[presentation=sheet]:rounded-t-xl data-[presentation=sheet]:ring-0 data-[presentation=sheet]:border-t data-[presentation=sheet]:pb-[max(1.5rem,env(safe-area-inset-bottom))] data-[presentation=sheet]:data-[state=open]:slide-in-from-bottom-full data-[presentation=sheet]:data-[state=closed]:slide-out-to-bottom-full data-[presentation=sheet]:data-[state=open]:zoom-in-100 data-[presentation=sheet]:data-[state=closed]:zoom-out-100";
                let base_class = format!("{base_one} {animations} {base_two} {sheet}");
                cn!(base_class.as_str(), class.get())
            }
        >
            <div class="flex flex-col gap-2">{children()}</div>
        </PopoverContentRoot>
    }
}

#[component]
pub fn PopoverHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="popover-header"
            class=move || cn!("flex flex-col px-4 gap-0.5 text-sm", &class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="popover-title"
            class=move || {
                cn!("font-medium leading-none tracking-tight text-foreground", &class.get())
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="popover-description"
            class=move || cn!("text-muted-foreground", &class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="popover-footer"
            class=move || {
                cn!(
                    "flex mt-auto items-center rounded-b-xl border-t bg-muted/50 p-4 group-data-[size=sm]/card:p-3", &class.get()
                )
            }
        >
            {children()}
        </div>
    }
}
