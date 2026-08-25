use crate::{
    cn,
    icons::chevron::ChevronDown,
    primitives::navigation_menu::{
        NavigationMenuContentRoot, NavigationMenuIndicatorRoot, NavigationMenuItemRoot,
        NavigationMenuLinkRoot, NavigationMenuListRoot, NavigationMenuRoot,
        NavigationMenuTriggerRoot,
    },
};
use leptos::{ev, prelude::*};

#[component]
pub fn NavigationMenu(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <NavigationMenuRoot
            value=value
            class=move || {
                cn!("relative flex max-w-max flex-1 items-center justify-center", class.get())
            }
        >
            {children()}
        </NavigationMenuRoot>
    }
}

#[component]
pub fn NavigationMenuList(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <NavigationMenuListRoot class=move || {
            cn!("group flex flex-1 list-none items-center justify-center gap-1", class.get())
        }>{children()}</NavigationMenuListRoot>
    }
}

#[component]
pub fn NavigationMenuItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <NavigationMenuItemRoot value=value class=class>
            {children()}
        </NavigationMenuItemRoot>
    }
}

#[component]
pub fn NavigationMenuTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <NavigationMenuTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "group/nav-trigger inline-flex h-9 w-max items-center justify-center gap-1 rounded-md bg-background px-3 py-2 text-sm font-medium outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50 data-[state=open]:text-accent-foreground",
                    class.get()
                )
            }
        >
            {children()}
            <ChevronDown class="relative top-px size-3.5 text-muted-foreground transition-transform duration-200 group-data-[state=open]/nav-trigger:rotate-180" />
        </NavigationMenuTriggerRoot>
    }
}

#[component]
pub fn NavigationMenuContent(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <NavigationMenuContentRoot class=move || {
            cn!(
                "absolute top-full left-0 z-50 mt-1.5 w-max overflow-hidden rounded-lg border bg-popover p-2 text-popover-foreground shadow-md animation-duration-200",
                "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=open]:slide-in-from-top-1",
                "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=closed]:pointer-events-none",
                class.get()
            )
        }>{children()}</NavigationMenuContentRoot>
    }
}

#[component]
pub fn NavigationMenuLink(
    #[prop(optional, into)] href: Signal<String>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] render: Option<Callback<Callback<ev::MouseEvent>, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <NavigationMenuLinkRoot
            href=href
            active=active
            render=render
            class=move || {
                cn!(
                    "flex flex-col gap-1 rounded-md p-2 text-sm leading-none no-underline outline-none transition-colors select-none hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 data-active:bg-accent/50 data-active:text-accent-foreground [&_p]:text-xs [&_p]:leading-snug [&_p]:text-muted-foreground",
                    class.get()
                )
            }
        >
            {match children {
                Some(children) => leptos::either::Either::Left(children()),
                None => leptos::either::Either::Right(""),
            }}
        </NavigationMenuLinkRoot>
    }
}

#[component]
pub fn NavigationMenuIndicator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <NavigationMenuIndicatorRoot class=move || {
            cn!(
                "pointer-events-none absolute top-full z-50 flex h-1.5 items-end justify-center overflow-hidden transition-[left,width] duration-200 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0",
                class.get()
            )
        }>
            <div class="relative top-[60%] size-2 rotate-45 rounded-tl-sm bg-border shadow-md" />
        </NavigationMenuIndicatorRoot>
    }
}
