use crate::{
    cn,
    primitives::context_menu::{ContextMenuContentRoot, ContextMenuRoot, ContextMenuTriggerRoot},
    utils::types::{Align, Side, SideOffset},
};
use leptos::prelude::*;

pub use crate::components::dropdown_menu::{
    DropdownMenuItem as ContextMenuItem, DropdownMenuLabel as ContextMenuLabel,
    DropdownMenuPortal as ContextMenuPortal, DropdownMenuSeparator as ContextMenuSeparator,
    DropdownMenuSub as ContextMenuSub, DropdownMenuSubContent as ContextMenuSubContent,
    DropdownMenuSubTrigger as ContextMenuSubTrigger,
};

#[component]
pub fn ContextMenu(children: ChildrenFn) -> impl IntoView {
    view! { <ContextMenuRoot>{children()}</ContextMenuRoot> }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerRoot
            disabled=disabled
            class=move || cn!("select-none", class.get())
        >
            {children()}
        </ContextMenuTriggerRoot>
    }
}

#[component]
pub fn ContextMenuContent(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = Align::Start)] align: Align,
    #[prop(default = SideOffset(0.0))] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ContextMenuContentRoot
            side=side
            align=align
            side_offset=side_offset
            class=move || {
                cn!(
                    "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md pointer-events-auto data-[state=closed]:pointer-events-none",
                    "data-[state=open]:animate-in data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
                    "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                    class.get()
                )
            }
        >
            {children()}
        </ContextMenuContentRoot>
    }
}
