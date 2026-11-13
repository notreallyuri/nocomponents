use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn X(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </Icon>
    }
}
