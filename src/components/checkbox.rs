use crate::{
    cn,
    icons::{check::Check, minus::Minus},
    primitives::checkbox::CheckboxRoot,
    utils::{controllable::forward, icon::IconSlot},
};
use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] checked: Option<RwSignal<bool>>,
    #[prop(optional)] default_checked: bool,
    #[prop(optional, into)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] indeterminate_icon: Option<ViewFn>,
) -> impl IntoView {
    view! {
        <CheckboxRoot
            id=id
            indeterminate=indeterminate
            disabled=disabled
            name=name
            value=value
            required=required
            form=form
            checked=checked
            default_checked=default_checked
            on_checked_change=forward(on_checked_change)
            class=move || {
                cn!(
                    "group/checkbox peer size-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground data-[state=indeterminate]:bg-primary data-[state=indeterminate]:text-primary-foreground",
                    class.get()
                )
            }
        >
            <IconSlot
                icon=icon
                fallback=|| view! { <Check /> }
                data_slot="checkbox-indicator"
                class="hidden size-full items-center justify-center text-primary-foreground group-data-[state=checked]/checkbox:flex [&>svg]:size-full"
            />
            <IconSlot
                icon=indeterminate_icon
                fallback=|| view! { <Minus /> }
                data_slot="checkbox-indeterminate-indicator"
                class="hidden size-full items-center justify-center text-primary-foreground group-data-[state=indeterminate]/checkbox:flex [&>svg]:size-full"
            />
        </CheckboxRoot>
    }
}
