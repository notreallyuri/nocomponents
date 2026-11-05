use crate::{primitives::field::FieldControl, utils::controllable::controllable};
use leptos::{ev, prelude::*};

#[component]
pub fn SwitchRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether it is checked, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    checked: Option<RwSignal<bool>>,
    /// Whether it starts checked. Ignored when `checked` is given.
    #[prop(optional)]
    default_checked: bool,
    /// Called with the new value whenever it is checked or unchecked, however that came about.
    #[prop(optional, into)]
    on_checked_change: Option<Callback<bool>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked = controllable(checked, default_checked, on_checked_change);
    let field = FieldControl::new(id, disabled);

    let toggle = move |_: ev::MouseEvent| {
        if !field.disabled.get() {
            checked.update(|c| *c = !*c);
        }
    };

    view! {
        <button
            data-slot="switch"
            type="button"
            id=move || field.id.get()
            role="switch"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=move || field.disabled.get()
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            on:click=toggle
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
