use crate::{
    cn,
    icons::chevron::ChevronDown,
    primitives::accordion::{
        AccordionContentRoot, AccordionItemRoot, AccordionRoot, AccordionTriggerRoot, AccordionType,
    },
};
use leptos::prelude::*;

#[component]
pub fn Accordion(
    #[prop(optional)] kind: AccordionType,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Whether a `Single` accordion can be closed entirely. Ignored for `Multiple`.
    #[prop(default = true)]
    collapsible: bool,
    /// Omit for an uncontrolled accordion that owns its own selection.
    #[prop(default = None, into)]
    value: Option<RwSignal<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    view! {
        <AccordionRoot
            kind=kind
            disabled=disabled
            collapsible=collapsible
            value=value
            class=move || cn!("w-full", class.get())
        >
            {children()}
        </AccordionRoot>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <AccordionItemRoot
            value=value
            disabled=disabled
            class=move || cn!("border-b last:border-b-0", class.get())
        >
            {children()}
        </AccordionItemRoot>
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <AccordionTriggerRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex flex-1 items-center justify-between gap-4 py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-3 focus-visible:ring-ring/50 data-disabled:pointer-events-none data-disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
                    class.get()
                )
            }
        >
            {children()} <ChevronDown class="size-4 shrink-0 text-muted-foreground transition-transform duration-200" />
        </AccordionTriggerRoot>
    }
}

#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <AccordionContentRoot class="overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down">
            <div class=move || cn!("pt-0 pb-4 text-muted-foreground", class.get())>{children()}</div>
        </AccordionContentRoot>
    }
}
