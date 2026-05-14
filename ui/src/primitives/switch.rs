use leptos::{ev, prelude::*};

#[component]
pub fn SwitchRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional)] children: Option<Children>,
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
            role="switch"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=disabled
            on:click=toggle
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
