//! The browser's own `<select>`, bound to a signal.
//!
//! Distinct from `select.rs`, which builds a listbox out of divs. This one costs nothing, keeps
//! `optgroup`, and opens the platform picker on a phone. When its `<form>` is reset it goes back to
//! `default_value` (else `value`), read at that moment, and writes that into `model`.

use crate::{
    primitives::field::FieldControl,
    utils::form::{non_empty, on_native_reset},
};
use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn NativeSelectRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Two-way binding. Omit it and pass `value` for a one-way one.
    #[prop(default = None, into)]
    model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    /// What a reset of the form puts back, read when the reset happens. Empty means unset, and
    /// the current `value` is used. Never the `model`'s own earlier contents.
    #[prop(optional, into)]
    default_value: Signal<String>,
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
    /// Called with the chosen option's value whenever the reader changes it, whether or not
    /// `model` is bound.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// `<option>` and `<optgroup>` elements, written out as they are.
    children: Children,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);
    let node_ref = AnyNodeRef::new();
    on_native_reset(node_ref, form, model, default_value, value);

    view! {
        <select
            node_ref=node_ref
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
                let chosen = event_target_value(&e);
                if let Some(model) = model {
                    model.set(chosen.clone());
                }
                if let Some(on_change) = on_change {
                    on_change.run(chosen);
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

/// An `<option>` of a [`NativeSelectRoot`]. Written as a part rather than as a bare element so the
/// two layers name it the same way.
#[component]
pub fn NativeSelectOptionRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// What the option submits and what `model` holds while it is chosen. Empty falls back to the
    /// option's text, as a bare `<option>` does.
    #[prop(optional, into)]
    value: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <option
            data-slot="native-select-option"
            value=move || non_empty(value)
            disabled=move || disabled.get()
            class=class
        >
            {children()}
        </option>
    }
}

/// An `<optgroup>` of a [`NativeSelectRoot`], with a `label` the browser draws as its heading.
#[component]
pub fn NativeSelectOptGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] label: Signal<String>,
    /// Disables every option inside it.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <optgroup
            data-slot="native-select-optgroup"
            label=move || label.get()
            disabled=move || disabled.get()
            class=class
        >
            {children()}
        </optgroup>
    }
}
