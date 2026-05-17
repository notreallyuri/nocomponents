use leptos::prelude::*;

pub mod check;
pub mod chevron;
pub mod ellipsis;
pub mod loader;
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
