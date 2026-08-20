//! The trail back up.
//!
//! Style-only, but the markup matters: a `<nav>` labelled "breadcrumb" around an ordered list, the
//! current page marked `aria-current="page"` rather than linked, and the separators hidden from
//! assistive tech — they are punctuation, not content.

use crate::{cn, icons::chevron::ChevronRight};
use leptos::prelude::*;

#[component]
pub fn Breadcrumb(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav aria-label="breadcrumb" data-slot="breadcrumb" class=class>
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
    children: Children,
) -> impl IntoView {
    view! {
        <a
            href=move || href.get()
            data-slot="breadcrumb-link"
            class=move || {
                cn!("transition-colors hover:text-foreground", class.get())
            }
        >
            {children()}
        </a>
    }
}

/// The page you are on: not a link, and announced as the current one.
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
    /// Defaults to a chevron.
    #[prop(optional)]
    children: Option<Children>,
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
                None => view! { <ChevronRight /> }.into_any(),
            }}
        </li>
    }
}

/// Stands in for the middle of a long trail.
#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <span
            role="presentation"
            aria-hidden="true"
            data-slot="breadcrumb-ellipsis"
            class=move || cn!("flex size-5 items-center justify-center", class.get())
        >
            "…"
            <span class="sr-only">"More"</span>
        </span>
    }
}
