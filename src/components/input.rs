use crate::{cn, primitives::field::FieldControl, utils::form::non_empty};
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    File,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::File => "file",
        }
    }
}

#[component]
pub fn Input(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] input_type: Signal<InputType>,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] autocomplete: Signal<String>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(optional, into)] on_input: Option<Callback<String>>,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);

    view! {
        <input
            data-slot="input"
            type=move || input_type.get().as_str()
            id=move || field.id.get()
            disabled=move || field.disabled.get()
            name=move || non_empty(name)
            form=move || non_empty(form)
            required=move || required.get()
            readonly=move || readonly.get()
            placeholder=move || non_empty(placeholder)
            autocomplete=move || non_empty(autocomplete)
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            prop:value=move || { if let Some(m) = model { m.get() } else { value.get() } }
            on:input=move |ev| {
                let text = event_target_value(&ev);
                if let Some(m) = model {
                    m.set(text.clone());
                }
                if let Some(cb) = on_input {
                    cb.run(text);
                }
            }
            on:change=move |ev| {
                if let Some(cb) = on_change {
                    cb.run(event_target_value(&ev));
                }
            }
            class=move || {
                cn!(
                    "h-8 w-full min-w-0 rounded-lg border border-input bg-transparent px-2.5 py-1 text-base transition-colors outline-none file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-input/50 disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 md:text-sm dark:bg-input/30 dark:disabled:bg-input/80 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40", &class.get()
                )
            }
        />
    }
}
