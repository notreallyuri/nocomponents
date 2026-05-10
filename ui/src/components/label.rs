use crate::utils::cn;
use leptos::prelude::*;

#[component]
pub fn Label(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] html_for: Signal<String>,
    children: Children,
) -> impl IntoView {
    let base_class = "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50";

    view! {
        <label for=html_for class=move || { cn(base_class, &class.get()) }>
            {children()}
        </label>
    }
}
