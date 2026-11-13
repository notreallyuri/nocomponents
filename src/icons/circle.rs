use crate::{cn, icons::icon::Icon};
use leptos::prelude::*;

#[component]
pub fn Circle(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 24 24" stroke_width=2.0 class=move || cn!("size-4", class.get())>
            <circle cx="12" cy="12" r="10" />
        </Icon>
    }
}
