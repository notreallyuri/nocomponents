use crate::{cn, primitives::switch::SwitchRoot, utils::controllable::forward};
use leptos::prelude::*;

#[component]
pub fn Switch(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] checked: Option<RwSignal<bool>>,
    #[prop(optional)] default_checked: bool,
    #[prop(optional, into)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] form: Signal<String>,
) -> impl IntoView {
    view! {
        <SwitchRoot
            checked=checked
            default_checked=default_checked
            on_checked_change=forward(on_checked_change)
            disabled=disabled
            name=name
            value=value
            required=required
            form=form
            id=id
            class=move || {
                cn!(
                    "group/switch peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
                    class.get()
                )
            }
        >
            <span class="pointer-events-none block h-5 w-5 translate-x-0 rounded-full bg-background shadow-lg ring-0 transition-transform group-data-[state=checked]/switch:translate-x-5" />
        </SwitchRoot>
    }
}
