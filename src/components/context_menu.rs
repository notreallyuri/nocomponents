use crate::{
    cn,
    components::dropdown_menu::{MENU_SHEET_CLASS, SHEET_CLOSE_ICON},
    icons::x::X,
    primitives::{
        context_menu::{ContextMenuContentRoot, ContextMenuRoot, ContextMenuTriggerRoot},
        long_press::DEFAULT_DELAY_MS,
    },
    utils::{controllable::forward, icon::slot_view},
};
use leptos::prelude::*;

#[allow(unused_imports)]
pub use crate::components::dropdown_menu::{
    DropdownMenuCheckboxItem as ContextMenuCheckboxItem, DropdownMenuGroup as ContextMenuGroup,
    DropdownMenuItem as ContextMenuItem, DropdownMenuLabel as ContextMenuLabel,
    DropdownMenuPortal as ContextMenuPortal, DropdownMenuRadioGroup as ContextMenuRadioGroup,
    DropdownMenuRadioItem as ContextMenuRadioItem, DropdownMenuSeparator as ContextMenuSeparator,
    DropdownMenuShortcut as ContextMenuShortcut, DropdownMenuSub as ContextMenuSub,
    DropdownMenuSubContent as ContextMenuSubContent,
    DropdownMenuSubTrigger as ContextMenuSubTrigger,
};

#[allow(unused_imports)]
pub use crate::utils::types::{Align, Side, SideOffset};

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
    #[prop(default = true.into(), into)] long_press: Signal<bool>,
    #[prop(default = DEFAULT_DELAY_MS.into(), into)] long_press_ms: Signal<f64>,
    children: Children,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerRoot
            disabled=disabled
            long_press=long_press
            long_press_ms=long_press_ms
            class=move || cn!("select-none", class.get())
        >
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
    #[prop(optional, into)] mobile_sheet: Signal<bool>,
    #[prop(optional, into)] sheet_title: Signal<String>,
    #[prop(optional, into)] sheet_snap_points: Signal<Vec<f64>>,
    #[prop(default = true.into(), into)] sheet_show_close: Signal<bool>,
    #[prop(default = "Close".into(), into)] sheet_close_label: Signal<String>,
    #[prop(optional, into)] sheet_close_icon: Option<ViewFn>,
    #[prop(default = true.into(), into)] sheet_draggable: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <ContextMenuContentRoot
            side=side
            align=align
            side_offset=side_offset
            mobile_sheet=mobile_sheet
            sheet_title=sheet_title
            sheet_snap_points=sheet_snap_points
            sheet_show_close=sheet_show_close
            sheet_draggable=sheet_draggable
            sheet_close_label=sheet_close_label
            sheet_close_icon=Some(
                slot_view(sheet_close_icon, ViewFn::from(|| view! { <X /> }), SHEET_CLOSE_ICON),
            )
            class=move || {
                cn!(
                    "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md pointer-events-auto data-[state=closed]:pointer-events-none",
                    "data-[state=open]:animate-in data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
                    "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                    MENU_SHEET_CLASS,
                    class.get()
                )
            }
        >
            {children()}
        </ContextMenuContentRoot>
    }
}
