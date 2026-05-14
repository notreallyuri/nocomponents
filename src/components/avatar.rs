use crate::{
    cn,
    primitives::avatar::{AvatarFallbackRoot, AvatarImageRoot, AvatarRoot},
};
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl AvatarSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarSize::Default => "default",
            AvatarSize::Sm => "sm",
            AvatarSize::Lg => "lg",
        }
    }
}

#[component]
pub fn Avatar(
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <AvatarRoot>
            <span
                data-slot="avatar"
                data-size=size.as_str()
                class=move || {
                    cn!(
                        "group/avatar relative flex size-8 shrink-0 overflow-hidden rounded-full select-none data-[size=lg]:size-10 data-[size=sm]:size-6",
                    class.get()
                    )
                }
            >
                {children()}
            </span>
        </AvatarRoot>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: String,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <AvatarImageRoot
            src=src
            alt=alt
            data_slot="avatar-image"
            class=move || cn!("aspect-square size-full", class.get())
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(default = None, into)] delay_ms: Option<u64>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <AvatarFallbackRoot delay_ms=delay_ms>
            <span
                data-slot="avatar-fallback"
                class=move || {
                    cn!(
                        "flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground group-data-[size=sm]/avatar:text-xs",
                    class.get()
                    )
                }
            >
                {children()}
            </span>
        </AvatarFallbackRoot>
    }
}

#[component]
pub fn AvatarBadge(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="avatar-badge"
            class=move || {
                cn!(
                    "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-primary text-primary-foreground ring-2 ring-background select-none",
                "group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&>svg]:hidden",
                "group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&>svg]:size-2",
                "group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&>svg]:size-2",
                class.get()
                )
            }
        >
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="avatar-group"
            class=move || {
                cn!(
                    "group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background",
                class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarGroupCount(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="avatar-group-count"
            class=move || {
                cn!(
                    "relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-sm text-muted-foreground ring-2 ring-background group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3",
                class.get()
                )
            }
        >
            {children()}
        </div>
    }
}
