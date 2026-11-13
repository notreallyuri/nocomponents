use crate::{
    cn,
    icons::loader::{Loader, LoaderCircle},
    utils::icon::IconSlot,
};
use leptos::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum SpinnerVariant {
    #[default]
    Default,
    Lined,
}

#[derive(Default, Clone, Copy)]
pub enum SpinnerSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

impl SpinnerSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            SpinnerSize::Xs => "size-3",
            SpinnerSize::Sm => "size-3.5",
            SpinnerSize::Default => "size-4",
            SpinnerSize::Lg => "size-5",
            SpinnerSize::Icon => "size-4",
        }
    }
}

#[component]
pub fn Spinner(
    #[prop(optional, into)] variant: Signal<SpinnerVariant>,
    #[prop(optional, into)] size: Signal<SpinnerSize>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
) -> impl IntoView {
    let name = move || {
        let label = label.get();
        if label.is_empty() {
            "Loading".to_string()
        } else {
            label
        }
    };

    view! {
        <span
            data-slot="spinner"
            role="status"
            aria-label=name
            class=move || {
                cn!(
                    "inline-flex size-4 shrink-0 animate-spin items-center justify-center",
                    size.get().as_class(),
                    class.get()
                )
            }
        >
            <IconSlot
                icon=icon
                fallback=move || {
                    move || match variant.get() {
                        SpinnerVariant::Default => view! { <LoaderCircle /> }.into_any(),
                        SpinnerVariant::Lined => view! { <Loader /> }.into_any(),
                    }
                }
                class="flex size-full items-center justify-center [&>svg]:size-full"
            />
        </span>
    }
}
