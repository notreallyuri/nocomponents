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
    #[prop(optional)] input_type: InputType,
    #[prop(optional)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let base_class = "h-8 w-full min-w-0 rounded-lg border border-input bg-transparent px-2.5 py-1 text-base transition-colors outline-none file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-input/50 disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 md:text-sm dark:bg-input/30 dark:disabled:bg-input/80 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40";

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
            class=move || { crate::utils::cn(base_class, &class.get()) }
        />
    }
}
