use leptos::{ev, prelude::*};

#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    children: Children,
) -> impl IntoView {
    let toggle = move |_: ev::MouseEvent| {
        if !disabled.get() {
            checked.update(|c| *c = !*c);
        }
    };

    view! {
        <button
            type="button"
            id=id
            role="checkbox"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=disabled
            on:click=toggle
            class=class
        >
            {children()}
        </button>
    }
}
