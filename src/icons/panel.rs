use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn PanelLeft(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <path d="M9 3v18" />
        </Icon>
    }
}
