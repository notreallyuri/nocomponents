use crate::{
    cn,
    icons::loader::{Loader, LoaderCircle},
};
use leptos::{either::Either, prelude::*};

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
) -> impl IntoView {
    let name = move || {
        let label = label.get();
        if label.is_empty() {
            "Loading".to_string()
        } else {
            label
        }
    };

    move || match variant.get() {
        SpinnerVariant::Default => Either::Left(view! {
            <LoaderCircle
                attr:data-slot="spinner"
                attr:role="status"
                attr:aria-label=name
                class=move || { cn!("size-4 animate-spin", size.get().as_class(), class.get()) }
            />
        }),
        SpinnerVariant::Lined => Either::Right(view! {
            <Loader
                attr:data-slot="spinner"
                attr:role="status"
                attr:aria-label=name
                class=move || { cn!("size-4 animate-spin", size.get().as_class(), class.get()) }
            />
        }),
    }
}
