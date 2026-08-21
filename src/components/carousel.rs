use crate::{
    cn,
    icons::chevron::{ChevronDown, ChevronLeft, ChevronRight, ChevronUp},
    primitives::carousel::{
        CarouselButtonRoot, CarouselContentRoot, CarouselItemRoot, CarouselRoot, CarouselStep,
        use_carousel,
    },
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn Carousel(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = false)] wrap: bool,
    #[prop(optional, into)] index: RwSignal<usize>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CarouselRoot
            orientation=orientation
            wrap=wrap
            index=index
            class=move || cn!("relative", class.get())
        >
            {children()}
        </CarouselRoot>
    }
}

#[component]
pub fn CarouselContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CarouselContentRoot class=move || {
            cn!(
                // The scroll container *is* the strip: snapping, smooth scrolling and the hidden
                // scrollbar are what make it read as a carousel rather than a scroller.
                "flex snap-x snap-mandatory scroll-smooth overflow-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
                "data-[orientation=vertical]:snap-y data-[orientation=vertical]:flex-col",
                class.get()
            )
        }>{children()}</CarouselContentRoot>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CarouselItemRoot class=move || {
            cn!("min-w-0 shrink-0 grow-0 basis-full snap-start pl-4", class.get())
        }>{children()}</CarouselItemRoot>
    }
}

#[component]
pub fn CarouselPrevious(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! { <CarouselButton step=CarouselStep::Previous class=class disabled=disabled /> }
}

#[component]
pub fn CarouselNext(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! { <CarouselButton step=CarouselStep::Next class=class disabled=disabled /> }
}

#[component]
fn CarouselButton(
    step: CarouselStep,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let ctx = use_carousel();
    let previous = step == CarouselStep::Previous;

    view! {
        <CarouselButtonRoot
            step=step
            disabled=disabled
            class=move || {
                cn!(
                    "absolute inline-flex size-8 items-center justify-center rounded-full border bg-background text-foreground shadow-xs transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
                    if previous {
                        "top-1/2 -left-4 -translate-y-1/2 data-[orientation=vertical]:-top-4 data-[orientation=vertical]:left-1/2 data-[orientation=vertical]:-translate-x-1/2 data-[orientation=vertical]:translate-y-0"
                    } else {
                        "top-1/2 -right-4 -translate-y-1/2 data-[orientation=vertical]:-bottom-4 data-[orientation=vertical]:left-1/2 data-[orientation=vertical]:top-auto data-[orientation=vertical]:right-auto data-[orientation=vertical]:-translate-x-1/2 data-[orientation=vertical]:translate-y-0"
                    },
                    class.get()
                )
            }
        >
            {match (previous, ctx.orientation) {
                (true, Orientation::Horizontal) => view! { <ChevronLeft /> }.into_any(),
                (false, Orientation::Horizontal) => view! { <ChevronRight /> }.into_any(),
                (true, Orientation::Vertical) => view! { <ChevronUp /> }.into_any(),
                (false, Orientation::Vertical) => view! { <ChevronDown /> }.into_any(),
            }}
            <span class="sr-only">
                {if previous { "Previous slide" } else { "Next slide" }}
            </span>
        </CarouselButtonRoot>
    }
}
