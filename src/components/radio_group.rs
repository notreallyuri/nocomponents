use crate::{
    cn,
    primitives::radio_group::{RadioGroupItemRoot, RadioGroupRoot},
    utils::{controllable::forward, types::Orientation},
};
use leptos::prelude::*;

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] value: Option<RwSignal<Option<String>>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] form: Signal<String>,
    children: Children,
) -> impl IntoView {
    let value = Some(value.unwrap_or_else(|| RwSignal::new(default_value)));

    view! {
        <RadioGroupRoot
            orientation=orientation
            disabled=disabled
            value=value
            name=name
            required=required
            form=form
            on_value_change=forward(on_value_change)
            class=move || {
                cn!(
                    "flex gap-3 data-[orientation=vertical]:flex-col data-[orientation=horizontal]:flex-row data-[orientation=horizontal]:items-center",
                    class.get()
                )
            }
        >
            {children()}
        </RadioGroupRoot>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
) -> impl IntoView {
    view! {
        <RadioGroupItemRoot
            value=value
            id=id
            disabled=disabled
            class=move || {
                cn!(
                    "peer flex aspect-square size-4 shrink-0 items-center justify-center rounded-full border border-input text-primary shadow-xs transition-colors outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-primary dark:bg-input/30",
                    class.get()
                )
            }
        >
            <div class="size-2 scale-0 rounded-full bg-primary transition-transform in-data-[state=checked]:scale-100" />
        </RadioGroupItemRoot>
    }
}
