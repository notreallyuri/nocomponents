use crate::{cn, utils::types::AsClass};
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

impl AsClass for AlertVariant {
    fn as_class(&self) -> &'static str {
        match self {
            AlertVariant::Default => "bg-card text-card-foreground",
            AlertVariant::Destructive => {
                "bg-card text-destructive *:data-[slot=alert-description]:text-destructive/90 [&>svg]:text-current"
            }
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="alert"
            data-slot="alert"
            class=move || {
                cn!(
                    "relative grid w-full grid-cols-[0_1fr] items-start gap-y-0.5 rounded-xl px-4 py-3 text-sm ring-1 ring-foreground/10 has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-3 [&>svg]:size-4 [&>svg]:translate-y-0.5",
                    variant.as_class(),
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="alert-title"
            class=move || {
                cn!("col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight", class.get())
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="alert-description"
            class=move || {
                cn!(
                    "col-start-2 grid justify-items-start gap-1 text-sm text-muted-foreground [&_p]:leading-relaxed",
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}
