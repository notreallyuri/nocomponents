use crate::{
    cn,
    primitives::radio_group::{RadioGroupItemRoot, RadioGroupRoot},
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] class: Signal<String>,
    /// Radios stack by default, the way a native group does; the arrow keys follow suit.
    #[prop(default = Orientation::Vertical)]
    orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Omit for an uncontrolled group that owns its own selection.
    #[prop(default = None, into)]
    value: Option<RwSignal<Option<String>>>,
    children: Children,
) -> impl IntoView {
    view! {
        <RadioGroupRoot
            orientation=orientation
            disabled=disabled
            value=value
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
