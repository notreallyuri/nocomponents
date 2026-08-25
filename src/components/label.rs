use crate::cn;
use leptos::prelude::*;

#[component]
pub fn Label(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <label data-slot="label" class=move || {
            cn!(
                "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50", &class.get()
            )
        }>{children()}</label>
    }
}
