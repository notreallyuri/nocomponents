use crate::{
    cn,
    primitives::tooltip::{TooltipContentRoot, TooltipPortalRoot, TooltipRoot, TooltipTriggerRoot},
    utils::controllable::forward,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[allow(unused_imports)]
pub use crate::utils::types::{Align, AnchorRender, Side, SideOffset};

#[component]
pub fn Tooltip(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into, default = 400u64.into())] delay: Signal<u64>,
    #[prop(into, default = 100u64.into())] close_delay: Signal<u64>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <TooltipRoot
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            delay=delay
            close_delay=close_delay
            class=move || cn!("inline-block", class.get())
        >
            {children()}
        </TooltipRoot>
    }
}

#[component]
pub fn TooltipTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<Callback<AnchorRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <TooltipTriggerRoot
            disabled=disabled
            render=render
            class=move || cn!("inline-flex w-fit", class.get())
        >
            {children.map(|children| children())}
        </TooltipTriggerRoot>
    }
}

#[component]
pub fn TooltipContent(
    #[prop(into, default = Side::Top.into())] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(into, default = SideOffset(6.0).into())] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None)] container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let stored_children = StoredValue::new(children);

    view! {
        <TooltipPortalRoot container=container>
            <TooltipContentRoot
                side=side
                align=align
                side_offset=side_offset
                class=move || {
                    cn!(
                        "w-fit rounded-md bg-foreground px-2 py-1 text-xs text-background animation-duration-100 fill-mode-forwards data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-1 data-[side=left]:slide-in-from-right-1 data-[side=right]:slide-in-from-left-1 data-[side=top]:slide-in-from-bottom-1",
                        class.get()
                    )
                }
            >
                {stored_children.with_value(|children| children())}
            </TooltipContentRoot>
        </TooltipPortalRoot>
    }
}
