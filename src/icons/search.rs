use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn Search(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <path d="m21 21-4.34-4.34" />
            <circle cx="11" cy="11" r="8" />
        </Icon>
    }
}
