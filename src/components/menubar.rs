use crate::{
    cn,
    primitives::{
        dropdown_menu::{
            DropdownMenuContentRoot, DropdownMenuItemRoot, DropdownMenuPortalRoot,
            DropdownMenuSubContentRoot, DropdownMenuSubRoot, DropdownMenuSubTriggerRoot,
        },
        menubar::{MenubarMenuRoot, MenubarRoot, MenubarTriggerRoot, use_menubar},
    },
    utils::types::{Align, Side, SideOffset},
};
use leptos::{ev, portal::Portal, prelude::*};

#[component]
pub fn Menubar(
    #[prop(optional, into)] open: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <MenubarRoot
            open=open
            class=move || {
                cn!(
                    "flex h-9 items-center gap-1 rounded-lg border bg-background p-1 shadow-xs",
                    class.get()
                )
            }
        >
            {children()}
        </MenubarRoot>
    }
}

#[component]
pub fn MenubarMenu(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenubarMenuRoot value=value class=move || cn!("relative", class.get())>
            {children()}
        </MenubarMenuRoot>
    }
}

#[component]
pub fn MenubarTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <MenubarTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex select-none items-center rounded-md px-2 py-1 text-sm font-medium outline-none focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent data-[state=open]:text-accent-foreground hover:bg-accent hover:text-accent-foreground",
                    class.get()
                )
            }
        >
            {children()}
        </MenubarTriggerRoot>
    }
}

#[component]
pub fn MenubarPortal(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuPortalRoot children=children /> }
}

#[component]
pub fn MenubarContent(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let bar = use_menubar();

    view! {
        <DropdownMenuContentRoot
            side=side
            align=align
            side_offset=side_offset
            on_arrow_horizontal=Callback::new(move |direction| bar.step(direction))
            class=move || {
                cn!(
                    "z-50 min-w-48 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md pointer-events-auto data-[state=closed]:pointer-events-none",
                    "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
                    "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                    class.get()
                )
            }
        >
            {children()}
        </DropdownMenuContentRoot>
    }
}

#[component]
pub fn MenubarItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuItemRoot
            disabled=disabled
            on_click=on_click
            class=move || {
                cn!(
                    "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm font-medium outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            {children()}
        </DropdownMenuItemRoot>
    }
}

#[component]
pub fn MenubarLabel(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            cn!("px-2 py-1.5 text-xs font-semibold text-muted-foreground", class.get())
        }>{children()}</div>
    }
}

#[component]
pub fn MenubarSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! { <div class=move || cn!("-mx-1 my-1 h-px bg-muted", class.get()) /> }
}

#[component]
pub fn MenubarShortcut(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=move || {
            cn!("ml-auto text-xs tracking-widest text-muted-foreground", class.get())
        }>{children()}</span>
    }
}

#[component]
pub fn MenubarSub(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuSubRoot>{children()}</DropdownMenuSubRoot> }
}

#[component]
pub fn MenubarSubTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuSubTriggerRoot class=move || {
            cn!(
                "group flex w-full cursor-default select-none items-center justify-between gap-2 rounded-sm px-2 py-1.5 text-sm font-medium outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
                class.get()
            )
        }>
            {children()} <crate::icons::chevron::ChevronRight class="ml-auto" />
        </DropdownMenuSubTriggerRoot>
    }
}

#[component]
pub fn MenubarSubContent(
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = crate::primitives::dropdown_menu::use_dropdown();
    let children = StoredValue::new(children);

    view! {
        <Portal>
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
                    {children.with_value(|children| children())}
                </DropdownMenuSubContentRoot>
            </Show>
        </Portal>
    }
}
