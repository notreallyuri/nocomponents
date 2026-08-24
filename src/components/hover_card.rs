use crate::{
    cn,
    primitives::hover_card::{
        HoverCardContentRoot, HoverCardPortalRoot, HoverCardRoot, HoverCardTriggerRoot,
    },
    utils::types::{Align, Side, SideOffset},
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn HoverCard(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = 600, into)] delay: u64,
    #[prop(default = 250, into)] close_delay: u64,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <HoverCardRoot
            delay=delay
            close_delay=close_delay
            class=move || cn!("inline-block", class.get())
        >
            {children()}
        </HoverCardRoot>
    }
}

#[component]
pub fn HoverCardTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] render: Option<Callback<AnyNodeRef, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <HoverCardTriggerRoot render=render class=move || cn!("inline-flex w-fit", class.get())>
            {children.map(|children| children())}
        </HoverCardTriggerRoot>
    }
}

#[component]
pub fn HoverCardContent(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(default = SideOffset(8.0))] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let stored_children = StoredValue::new(children);

    view! {
        <HoverCardPortalRoot>
            <HoverCardContentRoot
                side=side
                align=align
                side_offset=side_offset
                class=move || {
                    cn!(
                        "w-64 rounded-lg bg-popover p-4 text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10 animation-duration-100 outline-none pointer-events-auto fill-mode-forwards data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=closed]:pointer-events-none data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                        class.get()
                    )
                }
            >
                {stored_children.with_value(|children| children())}
            </HoverCardContentRoot>
        </HoverCardPortalRoot>
    }
}
