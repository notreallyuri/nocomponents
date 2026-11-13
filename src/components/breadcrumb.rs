use crate::{cn, icons::chevron::ChevronRight, utils::icon::IconSlot};
use leptos::{either::Either, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[allow(unused_imports)]
pub use crate::utils::types::StyledRender;

#[component]
pub fn Breadcrumb(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] aria_label: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav
            aria-label=move || {
                let label = aria_label.get();
                if label.is_empty() { "Breadcrumb".to_string() } else { label }
            }
            data-slot="breadcrumb"
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ol
            data-slot="breadcrumb-list"
            class=move || {
                cn!(
                    "flex flex-wrap items-center gap-1.5 text-sm break-words text-muted-foreground sm:gap-2.5",
                    class.get()
                )
            }
        >
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            data-slot="breadcrumb-item"
            class=move || cn!("inline-flex items-center gap-1.5", class.get())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional, into)] href: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] render: Option<Callback<StyledRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Signal::derive(move || cn!("transition-colors hover:text-foreground", class.get()));
    let node_ref = AnyNodeRef::new();

    match render {
        Some(render) => Either::Left(render.run(StyledRender {
            class,
            node_ref,
            disabled: Signal::default(),
        })),
        None => Either::Right(view! {
            <a
                href=move || Some(href.get()).filter(|href| !href.is_empty())
                data-slot="breadcrumb-link"
                node_ref=node_ref
                class=class
            >
                {children.map(|children| children())}
            </a>
        }),
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            role="link"
            aria-disabled="true"
            aria-current="page"
            data-slot="breadcrumb-page"
            class=move || cn!("font-normal text-foreground", class.get())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li
            role="presentation"
            aria-hidden="true"
            data-slot="breadcrumb-separator"
            class=move || cn!("[&>svg]:size-3.5", class.get())
        >
            {match children {
                Some(children) => children(),
                None => {
                    view! {
                        <IconSlot
                            icon=icon
                            fallback=|| view! { <ChevronRight /> }
                            class="flex size-3.5 items-center justify-center [&>svg]:size-full"
                        />
                    }
                        .into_any()
                }
            }}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <span
            data-slot="breadcrumb-ellipsis"
            class=move || cn!("flex size-5 items-center justify-center", class.get())
        >
            <span aria-hidden="true">"…"</span>
            <span class="sr-only">"More"</span>
        </span>
    }
}
