use crate::{
    cn,
    primitives::context_menu::{ContextMenuContentRoot, ContextMenuRoot, ContextMenuTriggerRoot},
    utils::{
        controllable::forward,
        types::{Align, Side, SideOffset},
    },
};
use leptos::prelude::*;

pub use crate::components::dropdown_menu::{
    DropdownMenuCheckboxItem as ContextMenuCheckboxItem, DropdownMenuGroup as ContextMenuGroup,
    DropdownMenuItem as ContextMenuItem, DropdownMenuLabel as ContextMenuLabel,
    DropdownMenuPortal as ContextMenuPortal, DropdownMenuRadioGroup as ContextMenuRadioGroup,
    DropdownMenuRadioItem as ContextMenuRadioItem, DropdownMenuSeparator as ContextMenuSeparator,
    DropdownMenuShortcut as ContextMenuShortcut, DropdownMenuSub as ContextMenuSub,
    DropdownMenuSubContent as ContextMenuSubContent,
    DropdownMenuSubTrigger as ContextMenuSubTrigger,
};

#[component]
pub fn ContextMenu(
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ContextMenuRoot open=open default_open=default_open on_open_change=forward(on_open_change)>
            {children()}
        </ContextMenuRoot>
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerRoot disabled=disabled class=move || cn!("select-none", class.get())>
            {children()}
        </ContextMenuTriggerRoot>
    }
}

#[component]
pub fn ContextMenuContent(
    #[prop(into, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, default = SideOffset(0.0).into())] side_offset: Signal<SideOffset>,
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
