use crate::{
    cn,
    icons::{check::Check, chevron::ChevronDown, x::X},
    primitives::select::{
        SelectContentRoot, SelectItemRoot, SelectPortalRoot, SelectRoot, SelectTriggerRoot,
        use_select_label,
    },
    utils::{
        controllable::forward,
        icon::{IconSlot, slot_view},
    },
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Select(
    #[prop(default = None, into)] value: Option<RwSignal<Option<String>>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let value = Some(value.unwrap_or_else(|| RwSignal::new(default_value)));

    view! {
        <SelectRoot
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            value=value
            name=name
            required=required
            disabled=disabled
            form=form
            on_value_change=forward(on_value_change)
            class=move || cn!("relative w-full", class.get())
        >
            {children()}
        </SelectRoot>
    }
}

const SHEET_CLOSE_ICON: &str = "flex size-4 items-center justify-center [&>svg]:size-full";

#[component]
pub fn SelectTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <SelectTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex h-8 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                "aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40",
                class.get()
                )
            }
        >
            {children()}
            <IconSlot
                icon=icon
                fallback=|| view! { <ChevronDown /> }
                data_slot="select-icon"
                class="flex size-4 shrink-0 items-center justify-center text-muted-foreground [&>svg]:size-full"
            />
        </SelectTriggerRoot>
    }
}

#[component]
pub fn SelectValue(#[prop(into)] placeholder: Signal<String>) -> impl IntoView {
    let label = use_select_label();

    view! { <span class="truncate">{move || label.get().unwrap_or_else(|| placeholder.get())}</span> }
}

#[component]
pub fn SelectPortal(
    #[prop(default = None)] container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <SelectPortalRoot container=container children=children /> }
}

#[component]
pub fn SelectContent(
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
        <SelectContentRoot
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
                    "z-50 flex max-h-96 flex-col overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md pointer-events-auto data-[state=closed]:pointer-events-none",
                "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=top]:slide-in-from-bottom-2",
                "data-[presentation=sheet]:w-full data-[presentation=sheet]:max-h-[85dvh] data-[presentation=sheet]:rounded-b-none data-[presentation=sheet]:rounded-t-xl data-[presentation=sheet]:border-x-0 data-[presentation=sheet]:border-b-0 data-[presentation=sheet]:px-3 data-[presentation=sheet]:pb-[max(1rem,env(safe-area-inset-bottom))] data-[presentation=sheet]:data-[state=open]:slide-in-from-bottom-full data-[presentation=sheet]:data-[state=closed]:slide-out-to-bottom-full data-[presentation=sheet]:data-[state=open]:zoom-in-100 data-[presentation=sheet]:data-[state=closed]:zoom-out-100 data-[presentation=sheet]:[&_[data-slot=select-item]]:min-h-11 [&_[data-slot=menu-sheet-title]]:px-1 [&_[data-slot=menu-sheet-title]]:pb-2 [&_[data-slot=menu-sheet-title]]:text-sm [&_[data-slot=menu-sheet-title]]:font-semibold [&_[data-slot=menu-sheet-title]]:text-foreground [&_[data-slot=menu-sheet-close]]:flex [&_[data-slot=menu-sheet-close]]:size-8 [&_[data-slot=menu-sheet-close]]:items-center [&_[data-slot=menu-sheet-close]]:justify-center [&_[data-slot=menu-sheet-close]]:rounded-md [&_[data-slot=menu-sheet-close]]:text-muted-foreground [&_[data-slot=menu-sheet-close]]:outline-none [&_[data-slot=menu-sheet-close]]:hover:bg-accent [&_[data-slot=menu-sheet-close]]:hover:text-accent-foreground [&_[data-slot=menu-sheet-close]]:focus-visible:ring-2 [&_[data-slot=menu-sheet-close]]:focus-visible:ring-ring data-[presentation=sheet]:transition-transform data-[presentation=sheet]:duration-200 motion-reduce:data-[presentation=sheet]:transition-none data-[presentation=sheet]:data-[dragging=true]:transition-none",
                class.get()
                )
            }
        >
            <div class="min-h-0 w-full flex-1 overflow-y-auto overscroll-contain p-1">{children()}</div>
        </SelectContentRoot>
    }
}

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <SelectItemRoot
            value=value
            label=label
            disabled=disabled
            class=move || {
                cn!(
                    "group relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                <IconSlot
                    icon=icon
                    fallback=|| view! { <Check /> }
                    data_slot="select-item-indicator"
                    class="hidden size-4 shrink-0 items-center justify-center group-data-[state=checked]:flex [&>svg]:size-full"
                />
            </span>
            {children()}
        </SelectItemRoot>
    }
}
