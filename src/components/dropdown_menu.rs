use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::{check::Check, chevron::ChevronRight, circle::Circle},
    primitives::dropdown_menu::{
        DropdownMenuCheckboxItemRoot, DropdownMenuContentRoot, DropdownMenuGroupRoot,
        DropdownMenuItemRoot, DropdownMenuPortalRoot, DropdownMenuRadioGroupRoot,
        DropdownMenuRadioItemRoot, DropdownMenuRoot, DropdownMenuSubContentRoot,
        DropdownMenuSubRoot, DropdownMenuSubTriggerRoot, DropdownMenuTriggerRoot,
    },
    primitives::floating::FloatingPortal,
    utils::{controllable::forward, icon::IconSlot},
};
use leptos::{either::Either, ev, prelude::*};

#[allow(unused_imports)]
pub use crate::primitives::dropdown_menu::DropdownMenuItemVariant;
#[allow(unused_imports)]
pub use crate::utils::types::{Align, Side, SideOffset, TriggerRender};

pub const MENU_SHEET_CLASS: &str = "data-[presentation=sheet]:w-full data-[presentation=sheet]:max-h-[85dvh] data-[presentation=sheet]:overflow-y-auto data-[presentation=sheet]:rounded-b-none data-[presentation=sheet]:rounded-t-xl data-[presentation=sheet]:border-x-0 data-[presentation=sheet]:border-b-0 data-[presentation=sheet]:px-4 data-[presentation=sheet]:pb-[max(1rem,env(safe-area-inset-bottom))] data-[presentation=sheet]:data-[state=open]:slide-in-from-bottom-full data-[presentation=sheet]:data-[state=closed]:slide-out-to-bottom-full data-[presentation=sheet]:data-[state=open]:zoom-in-100 data-[presentation=sheet]:data-[state=closed]:zoom-out-100 data-[presentation=sheet]:[&_[data-slot=dropdown-menu-item]]:min-h-11";

#[component]
pub fn DropdownMenu(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <DropdownMenuRoot
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            class=move || cn!("relative inline-block text-left", class.get())
        >
            {children()}
        </DropdownMenuRoot>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] render: Option<Callback<TriggerRender, AnyView>>,
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

    view! { <DropdownMenuTriggerRoot disabled=disabled render=render.unwrap_or(styled) /> }
}

#[component]
pub fn DropdownMenuPortal(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuPortalRoot children=children /> }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] mobile_sheet: Signal<bool>,
    #[prop(optional, into)] sheet_title: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuContentRoot
            side=side
            align=align
            side_offset=side_offset
            mobile_sheet=mobile_sheet
            sheet_title=sheet_title
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
        </DropdownMenuContentRoot>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] variant: Signal<DropdownMenuItemVariant>,
    #[prop(default = true.into(), into)] close_on_select: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuItemRoot
            disabled=disabled
            inset=inset
            variant=variant
            close_on_select=close_on_select
            on_click=forward(on_click)
            class=move || {
                cn!(
                    "relative flex font-medium cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50 data-[inset]:pl-8 data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:hover:bg-destructive/10 data-[variant=destructive]:hover:text-destructive data-[variant=destructive]:*:[svg]:text-destructive",
                    class.get()
                )
            }
        >
            {children()}
        </DropdownMenuItemRoot>
    }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] inset: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="dropdown-menu-label"
            data-inset=move || inset.get().then_some("")
            class=move || {
                cn!(
                    "px-2 py-1.5 text-xs font-semibold text-muted-foreground data-[inset]:pl-8",
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <div
            data-slot="dropdown-menu-separator"
            role="separator"
            class=move || cn!("-mx-1 my-1 h-px bg-muted", class.get())
        ></div>
    }
}

#[component]
pub fn DropdownMenuGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! { <DropdownMenuGroupRoot class=class>{children()}</DropdownMenuGroupRoot> }
}

#[component]
pub fn DropdownMenuShortcut(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="dropdown-menu-shortcut"
            class=move || cn!("ml-auto text-xs tracking-widest text-muted-foreground", class.get())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] checked: RwSignal<bool>,
    #[prop(optional, into)] close_on_select: Signal<bool>,
    #[prop(optional, into)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuCheckboxItemRoot
            checked=checked
            close_on_select=close_on_select
            on_checked_change=forward(on_checked_change)
            disabled=disabled
            inset=inset
            class=move || {
                cn!(
                    "group relative flex cursor-default select-none items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm font-medium outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            <span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
                <IconSlot
                    icon=icon
                    fallback=|| view! { <Check /> }
                    data_slot="dropdown-menu-item-indicator"
                    class="hidden size-4 shrink-0 items-center justify-center group-data-[state=checked]:flex [&>svg]:size-full"
                />
            </span>
            {children()}
        </DropdownMenuCheckboxItemRoot>
    }
}

#[component]
pub fn DropdownMenuRadioGroup(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuRadioGroupRoot
            class=class
            value=value
            on_value_change=forward(on_value_change)
        >
            {children()}
        </DropdownMenuRadioGroupRoot>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] close_on_select: Signal<bool>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuRadioItemRoot
            value=value
            close_on_select=close_on_select
            disabled=disabled
            inset=inset
            class=move || {
                cn!(
                    "group relative flex cursor-default select-none items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm font-medium outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            <span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
                <IconSlot
                    icon=icon
                    fallback=|| view! { <Circle /> }
                    data_slot="dropdown-menu-item-indicator"
                    class="hidden size-2 shrink-0 items-center justify-center group-data-[state=checked]:flex [&>svg]:size-full [&>svg]:fill-current"
                />
            </span>
            {children()}
        </DropdownMenuRadioItemRoot>
    }
}

#[component]
pub fn DropdownMenuSub(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <DropdownMenuSubRoot class=class>{children()}</DropdownMenuSubRoot> }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] inset: Signal<bool>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuSubTriggerRoot
            disabled=disabled
            inset=inset
            class=move || {
                cn!(
                    "group flex w-full cursor-default select-none font-medium justify-between items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground data-[inset]:pl-8 disabled:pointer-events-none disabled:opacity-50",
                class.get()
                )
            }
        >
            {children()}
            <IconSlot
                icon=icon
                fallback=|| view! { <ChevronRight /> }
                data_slot="dropdown-menu-sub-icon"
                class="ml-auto flex size-4 shrink-0 items-center justify-center [&>svg]:size-full"
            />
        </DropdownMenuSubTriggerRoot>
    }
}

#[component]
pub fn DropdownMenuSubContent(
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = crate::primitives::dropdown_menu::use_dropdown();
    let children = StoredValue::new(children);

    view! {
        <FloatingPortal>
            <Show when=move || ctx.is_mounted.get()>
                <DropdownMenuSubContentRoot
                    side_offset=side_offset
                    class=move || {
                        cn!(
                            "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg pointer-events-auto data-[state=closed]:pointer-events-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                            class.get()
                        )
                    }
                >
                    {children.with_value(|c| c())}
                </DropdownMenuSubContentRoot>
            </Show>
        </FloatingPortal>
    }
}
