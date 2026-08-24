use crate::{cn, components::separator::Separator, utils::types::AsClass};
use leptos::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl AsClass for ItemVariant {
    fn as_class(&self) -> &'static str {
        match self {
            ItemVariant::Default => "bg-transparent",
            ItemVariant::Outline => "border-border",
            ItemVariant::Muted => "bg-muted/50",
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl AsClass for ItemSize {
    fn as_class(&self) -> &'static str {
        match self {
            ItemSize::Default => "gap-4 p-4",
            ItemSize::Sm => "gap-2.5 px-4 py-3",
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum ItemMediaVariant {
    #[default]
    Plain,
    Icon,
}

impl AsClass for ItemMediaVariant {
    fn as_class(&self) -> &'static str {
        match self {
            ItemMediaVariant::Plain => "bg-transparent",
            ItemMediaVariant::Icon => {
                "size-8 rounded-sm border bg-muted [&_svg:not([class*='size-'])]:size-4"
            }
        }
    }
}

#[component]
pub fn ItemGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-group"
            role="list"
            class=move || cn!("group/item-group flex flex-col", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! { <Separator class=move || cn!("my-0", class.get()) /> }
}

#[component]
pub fn Item(
    #[prop(optional)] variant: ItemVariant,
    #[prop(optional)] size: ItemSize,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.as_class();
    let size_class = size.as_class();

    view! {
        <div
            data-slot="item"
            data-variant=match variant {
                ItemVariant::Default => "default",
                ItemVariant::Outline => "outline",
                ItemVariant::Muted => "muted",
            }
            class=move || {
                cn!(
                    "group/item flex flex-wrap items-center rounded-lg border border-transparent text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 [a]:hover:bg-accent/50",
                    variant_class,
                    size_class,
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemMedia(
    #[prop(optional)] variant: ItemMediaVariant,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.as_class();

    view! {
        <div
            data-slot="item-media"
            class=move || {
                cn!(
                    "flex shrink-0 items-center justify-center gap-2 self-start text-muted-foreground [&_svg]:pointer-events-none",
                    variant_class,
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-content"
            class=move || {
                cn!(
                    "flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none",
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-title"
            class=move || {
                cn!("flex w-fit items-center gap-2 text-sm leading-snug font-medium", class.get())
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-description"
            class=move || {
                cn!(
                    "line-clamp-2 text-sm leading-normal font-normal text-balance text-muted-foreground",
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemActions(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-actions"
            class=move || cn!("flex items-center gap-2", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-header"
            class=move || {
                cn!("flex basis-full items-center justify-between gap-2", class.get())
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ItemFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-footer"
            class=move || {
                cn!("flex basis-full items-center justify-between gap-2", class.get())
            }
        >
            {children()}
        </div>
    }
}
