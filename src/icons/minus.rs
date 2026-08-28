use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Minus(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M5 12h14" />
        </IconRoot>
    }
}
