use crate::{
    cn,
    primitives::collapsible::{CollapsibleContentRoot, CollapsibleRoot, CollapsibleTriggerRoot},
};
use leptos::prelude::*;

#[component]
pub fn Collapsible(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    view! {
        <CollapsibleRoot
            disabled=disabled
            open=open
            class=move || cn!("flex flex-col gap-2", class.get())
        >
            {children()}
        </CollapsibleRoot>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <CollapsibleTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex w-full items-center justify-between gap-2 rounded-lg text-sm font-medium transition-colors outline-none focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50",
                    class.get()
                )
            }
        >
            {children()}
        </CollapsibleTriggerRoot>
    }
}

#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <CollapsibleContentRoot class=move || {
            cn!(
                "overflow-hidden text-sm data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down",
                class.get()
            )
        }>{children()}</CollapsibleContentRoot>
    }
}
