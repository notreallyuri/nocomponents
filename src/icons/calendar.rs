use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn Calendar(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="M8 2v4" />
            <path d="M16 2v4" />
            <rect width="18" height="18" x="3" y="4" rx="2" />
            <path d="M3 10h18" />
        </Icon>
    }
}
