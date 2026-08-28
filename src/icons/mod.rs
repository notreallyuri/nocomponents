//! The handful of icons the styled layer needs, drawn inline rather than pulled from a crate: a
//! UI library should not hand its consumers an icon dependency for six glyphs.
//!
//! The path data is [Lucide](https://lucide.dev)'s, under the ISC licence.

use leptos::prelude::*;

pub mod calendar;
pub mod check;
pub mod chevron;
pub mod copy;
pub mod ellipsis;
pub mod loader;
pub mod media;
pub mod message;
pub mod minus;
pub mod panel;
pub mod paperclip;
pub mod plus;
pub mod search;
pub mod x;

#[component]
pub fn IconRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="square"
            stroke-linejoin="miter"
            class=class
        >
            {children()}
        </svg>
    }
}
