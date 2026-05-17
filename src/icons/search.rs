use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Search(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="m21 21-4.34-4.34" />
            <circle cx="11" cy="11" r="8" />
        </IconRoot>
    }
}
