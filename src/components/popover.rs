use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    primitives::popover::{PopoverContentRoot, PopoverPortalRoot, PopoverRoot, use_popover},
    utils::types::{Align, Side, SideOffset},
};
use leptos::{either::Either, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Popover(children: ChildrenFn) -> impl IntoView {
    view! { <PopoverRoot>{children()}</PopoverRoot> }
}

#[component]
pub fn PopoverTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] as_child: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_popover();
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    match as_child {
        Some(render_fn) => Either::Left(render_fn.run((ctx.trigger_ref, on_click))),
        None => Either::Right(view! {
            <Button
                size=size
                variant=variant
                attr:disabled=disabled
                class=class
                node_ref=ctx.trigger_ref
                on_click=on_click
            >
                {
                    let children = children.clone();
                    move || match &children {
                        Some(child) => Either::Left(child()),
                        None => Either::Right(view! { "" }.into_any()),
                    }
                }
            </Button>
        }),
    }
}

#[component]
pub fn PopoverPortal(children: ChildrenFn) -> impl IntoView {
    view! { <PopoverPortalRoot children=children /> }
}

#[component]
pub fn PopoverContent(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <PopoverContentRoot
            side=side
            align=align
            side_offset=side_offset
            class=move || {
                let base_one = "flex flex-col gap-2 py-4 rounded-lg bg-popover text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10 outline-none duration-100 fill-mode-forwards pointer-events-auto data-[state=closed]:pointer-events-none";
                let animations = "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
                let base_two = "has-data-[slot=popover-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl";
                let base_class = format!("{base_one} {animations} {base_two}");
                cn!(base_class.as_str(), class.get())
            }
        >
            <div data-slot="popover-content" class="flex flex-col gap-2">
                {children()}
            </div>
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
