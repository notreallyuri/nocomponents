//! The browser's own `<select>`, bound to a signal.
//!
//! Distinct from `select.rs`, which builds a listbox out of divs. This one costs nothing, keeps
//! `optgroup`, and opens the platform picker on a phone.

use crate::primitives::field::FieldControl;
use leptos::{ev, prelude::*};

#[component]
pub fn NativeSelectRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Two-way binding. Omit it and pass `value` for a one-way one.
    #[prop(default = None, into)]
    model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    /// `<option>` and `<optgroup>` elements, written out as they are.
    children: Children,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);

    view! {
        <select
            id=move || field.id.get()
            disabled=move || field.disabled.get()
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            data-slot="native-select"
            prop:value=move || if let Some(model) = model { model.get() } else { value.get() }
            on:change=move |e: ev::Event| {
                if let Some(model) = model {
                    model.set(event_target_value(&e));
                }
            }
            class=class
        >
            {children()}
        </select>
    }
}
