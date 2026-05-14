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
pub fn InputRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] input_type: InputType,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            id=id
            type=input_type.as_str()
            name=name
            placeholder=placeholder
            prop:value=move || { if let Some(m) = model { m.get() } else { value.get() } }
            on:input=move |ev| {
                if let Some(m) = model {
                    m.set(event_target_value(&ev));
                }
            }
            disabled=disabled
            class=class
        />
    }
}
