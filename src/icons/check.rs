use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Check(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M20 6 9 17l-5-5" />
        </IconRoot>
    }
}
