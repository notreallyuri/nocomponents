use crate::primitives::field::FieldControl;
use leptos::{ev, prelude::*};

#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    children: Children,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);

    let toggle = move |_: ev::MouseEvent| {
        if !field.disabled.get() {
            checked.update(|c| *c = !*c);
        }
    };

    view! {
        <button
            data-slot="checkbox"
            type="button"
            id=move || field.id.get()
            role="checkbox"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=move || field.disabled.get()
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            on:click=toggle
            class=class
        >
            {children()}
        </button>
    }
}
