use crate::{cn, icons::IconRoot};
use leptos::prelude::*;

#[component]
pub fn Plus(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <IconRoot class=move || cn!("size-4", class.get())>
            <path d="M5 12h14M12 5v14" />
        </IconRoot>
    }
}
