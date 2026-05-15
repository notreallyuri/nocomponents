use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::dropdown_menu::{
        use_dropdown, DropdownMenuContentRoot, DropdownMenuItemRoot, DropdownMenuPortalRoot,
        DropdownMenuRoot, DropdownMenuSubContentRoot, DropdownMenuSubRoot,
        DropdownMenuSubTriggerRoot,
    },
    utils::types::{Align, Side, SideOffset},
};
use leptos::{either::Either, ev, portal::Portal, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn DropdownMenu(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuRoot class="relative inline-block text-left">{children()}</DropdownMenuRoot> }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] render: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dropdown();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match render {
        Some(render_fn) => Either::Left(render_fn.run((ctx.trigger_ref, on_click))),
        None => Either::Right(view! {
            <Button
                size=size
                variant=variant
                class=class
                disabled=disabled
                on_click=on_click
                node_ref=ctx.trigger_ref
            >
                {match children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(""),
                }}
            </Button>
        }),
    }
}

#[component]
pub fn DropdownMenuPortal(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuPortalRoot children=children /> }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuContentRoot
            side=side
            align=align
            side_offset=side_offset
            class=move || {
                cn!(
                    "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md",
                    "data-[state=open]:animate-in data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
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
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuItemRoot
            on_click=on_click
            class=move || {
                cn!(
                    "relative flex font-medium cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            {children()}
        </DropdownMenuItemRoot>
    }
}

#[component]
pub fn DropdownMenuLabel(children: Children) -> impl IntoView {
    view! { <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground">{children()}</div> }
}

#[component]
pub fn DropdownMenuSeparator() -> impl IntoView {
    view! { <div class="-mx-1 my-1 h-px bg-muted"></div> }
}

#[component]
pub fn DropdownMenuSub(children: ChildrenFn) -> impl IntoView {
    view! { <DropdownMenuSubRoot>{children()}</DropdownMenuSubRoot> }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <DropdownMenuSubTriggerRoot class=move || {
            cn!(
                "group flex w-full cursor-default select-none font-medium justify-between items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
                class.get()
            )
        }>
            {children()}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="ml-auto h-4 w-4 transition-transform duration-200 group-data-[state=open]:rotate-90"
            >
                <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
        </DropdownMenuSubTriggerRoot>
    }
}

#[component]
pub fn DropdownMenuSubContent(
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
                            "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                            class.get()
                        )
                    }
                >
                    {children.with_value(|c| c())}
                </DropdownMenuSubContentRoot>
            </Show>
        </Portal>
    }
}
