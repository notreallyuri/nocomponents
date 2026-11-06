use crate::{cn, utils::types::AsClass};
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    #[default]
    Default,
    Tile,
    Line,
    Card,
}

impl SkeletonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkeletonVariant::Default => "default",
            SkeletonVariant::Tile => "tile",
            SkeletonVariant::Line => "line",
            SkeletonVariant::Card => "card",
        }
    }
}

impl AsClass for SkeletonVariant {
    fn as_class(&self) -> &'static str {
        match self {
            SkeletonVariant::Default | SkeletonVariant::Card => "",
            SkeletonVariant::Tile => "aspect-square",
            SkeletonVariant::Line => "h-3 rounded-sm",
        }
    }
}

const BONE: &str = "animate-pulse rounded-md bg-muted motion-reduce:animate-none";

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] variant: Signal<SkeletonVariant>,
) -> impl IntoView {
    move || match variant.get() {
        SkeletonVariant::Card => view! {
            <div
                aria-hidden="true"
                data-slot="skeleton"
                data-variant="card"
                class=move || cn!("flex flex-col gap-2", class.get())
            >
                <div class=cn!(BONE, "aspect-square w-full") />
                <div class=cn!(BONE, "h-3 w-3/4 rounded-sm") />
                <div class=cn!(BONE, "h-2.5 w-1/2 rounded-sm") />
            </div>
        }
        .into_any(),
        shape => view! {
            <div
                aria-hidden="true"
                data-slot="skeleton"
                data-variant=shape.as_str()
                class=move || cn!(BONE, shape.as_class(), class.get())
            />
        }
        .into_any(),
    }
}
