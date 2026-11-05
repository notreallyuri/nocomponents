//! The browser's own `<select>`, bound to a signal.
//!
//! Distinct from `select.rs`, which builds a listbox out of divs. This one costs nothing, keeps
//! `optgroup`, and opens the platform picker on a phone.

use crate::{primitives::field::FieldControl, utils::form::non_empty};
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
    /// The name the choice is submitted under. Empty leaves it out of a form's data.
    #[prop(optional, into)]
    name: Signal<String>,
    /// The browser refuses to submit the form while the selected option's value is empty.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// The `id` of a `<form>` the select belongs to when it is not inside one. Empty means the
    /// one around it.
    #[prop(optional, into)]
    form: Signal<String>,
    /// Run when the control loses focus. A convenience now rather than a necessity: the styled
    /// layer used to wrap this in a positioned `<div>` for its chevron, where a caller's own
    /// `on:blur` landed on the wrapper and never fired, `blur` not bubbling. The chevron is a
    /// background image on the `<select>` itself these days, so an `on:blur` at the call site
    /// reaches the control.
    #[prop(optional, into)]
    on_blur: Option<Callback<ev::FocusEvent>>,
    /// `<option>` and `<optgroup>` elements, written out as they are.
    children: Children,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);

    view! {
        <select
            id=move || field.id.get()
            disabled=move || field.disabled.get()
            name=move || non_empty(name)
            form=move || non_empty(form)
            required=move || required.get()
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            data-slot="native-select"
            prop:value=move || if let Some(model) = model { model.get() } else { value.get() }
            on:change=move |e: ev::Event| {
                if let Some(model) = model {
                    model.set(event_target_value(&e));
                }
            }
            on:blur=move |e: ev::FocusEvent| {
                if let Some(on_blur) = on_blur {
                    on_blur.run(e);
                }
            }
            class=class
        >
            {children()}
        </select>
    }
}
