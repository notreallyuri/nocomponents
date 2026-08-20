//! Style-only wrappers over the native table elements.
//!
//! `Table` puts the `<table>` inside its own horizontally scrolling container, so a wide table
//! scrolls itself instead of stretching the page.

use crate::cn;
use leptos::prelude::*;

#[component]
pub fn Table(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div data-slot="table-container" class="relative w-full overflow-x-auto">
            <table
                data-slot="table"
                class=move || cn!("w-full caption-bottom text-sm", class.get())
            >
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead data-slot="table-header" class=move || cn!("[&_tr]:border-b", class.get())>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody data-slot="table-body" class=move || cn!("[&_tr:last-child]:border-0", class.get())>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFooter(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tfoot
            data-slot="table-footer"
            class=move || {
                cn!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", class.get())
            }
        >
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: Signal<String>,
    /// Renders `data-state="selected"`, which the row styles itself from.
    #[prop(optional, into)]
    selected: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            data-slot="table-row"
            data-state=move || if selected.get() { "selected" } else { "" }
            class=move || {
                cn!(
                    "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
                    class.get()
                )
            }
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional, into)] class: Signal<String>,
    /// Optional: an empty cell — a checkbox column's header, a spacer — is a normal thing to want.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <th
            data-slot="table-head"
            class=move || {
                cn!(
                    "h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground [&:has([role=checkbox])]:pr-0",
                    class.get()
                )
            }
        >
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: Signal<String>,
    /// Optional: an empty cell — a checkbox column's header, a spacer — is a normal thing to want.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <td
            data-slot="table-cell"
            class=move || {
                cn!(
                    "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0",
                    class.get()
                )
            }
        >
            {children.map(|c| c())}
        </td>
    }
}

#[component]
pub fn TableCaption(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <caption
            data-slot="table-caption"
            class=move || cn!("mt-4 text-sm text-muted-foreground", class.get())
        >
            {children()}
        </caption>
    }
}
