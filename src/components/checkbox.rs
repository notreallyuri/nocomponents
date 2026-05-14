use crate::{cn, primitives::checkbox::CheckboxRoot};
use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
) -> impl IntoView {
    view! {
        <CheckboxRoot
            id=id
            disabled=disabled
            checked=checked
            class=move || {
                cn!(
                    "peer size-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
                    class.get()
                )
            }
        >
            <div class=move || {
                cn!(
                    "flex items-center justify-center text-primary-foreground",
                    if checked.get() { "visible" } else { "hidden" }
                )
            }>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4"
                >
                    <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
            </div>
        </CheckboxRoot>
    }
}
