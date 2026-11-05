use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Circle(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <circle cx="12" cy="12" r="10" />
        </IconRoot>
    }
}
