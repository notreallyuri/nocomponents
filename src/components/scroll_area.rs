//! The styled scroll area.
//!
//! Sizes and positions come from `src/primitives/scroll_area.rs` as inline geometry; what is
//! decided here is what a track and a thumb look like, and that a scrollbar for content which
//! fits is not drawn at all.

use crate::{
    cn,
    primitives::scroll_area::{
        ScrollAreaRoot, ScrollAreaScrollbarRoot, ScrollAreaThumbRoot, ScrollAreaViewportRoot,
    },
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn ScrollArea(
    #[prop(optional, into)] class: Signal<String>,
    /// Also draw a horizontal scrollbar. Off by default: most scroll areas are vertical, and a
    /// horizontal track under content that never overflows sideways is just a line.
    #[prop(default = false)]
    horizontal: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <ScrollAreaRoot class=move || cn!("group/scroll-area relative overflow-hidden", class.get())>
            <ScrollAreaViewport>{children()}</ScrollAreaViewport>
            <ScrollAreaScrollbar orientation=Orientation::Vertical>
                <ScrollAreaThumb orientation=Orientation::Vertical />
            </ScrollAreaScrollbar>
            {horizontal
                .then(|| {
                    view! {
                        <ScrollAreaScrollbar orientation=Orientation::Horizontal>
                            <ScrollAreaThumb orientation=Orientation::Horizontal />
                        </ScrollAreaScrollbar>
                    }
                })}
        </ScrollAreaRoot>
    }
}

#[component]
pub fn ScrollAreaViewport(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ScrollAreaViewportRoot class=move || {
            cn!(
                // The primitive's inline style already sets `scrollbar-width: none`, which covers
                // Firefox and the standard property. WebKit needs a pseudo-element rule, and an
                // inline style cannot write one — hence this class.
                "h-full w-full [&::-webkit-scrollbar]:hidden",
                class.get()
            )
        }>{children()}</ScrollAreaViewportRoot>
    }
}

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ScrollAreaScrollbarRoot
            orientation=orientation
            class=move || {
                cn!(
                    "absolute flex touch-none p-0.5 opacity-0 transition-opacity select-none data-[state=hidden]:pointer-events-none",
                    // Shown on hover, or whenever the pointer is anywhere in the area — the same
                    // affordance an overlay scrollbar has on a trackpad.
                    "group-hover/scroll-area:opacity-100 hover:opacity-100 data-[state=visible]:opacity-60",
                    "data-[orientation=vertical]:top-0 data-[orientation=vertical]:right-0 data-[orientation=vertical]:h-full data-[orientation=vertical]:w-2.5",
                    "data-[orientation=horizontal]:bottom-0 data-[orientation=horizontal]:left-0 data-[orientation=horizontal]:h-2.5 data-[orientation=horizontal]:w-full data-[orientation=horizontal]:flex-col",
                    class.get()
                )
            }
        >
            {children()}
        </ScrollAreaScrollbarRoot>
    }
}

#[component]
pub fn ScrollAreaThumb(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ScrollAreaThumbRoot
            orientation=orientation
            class=move || {
                cn!(
                    "relative rounded-full bg-border transition-colors hover:bg-muted-foreground/60 data-[state=dragging]:bg-muted-foreground/80",
                    "data-[orientation=vertical]:w-full data-[orientation=horizontal]:h-full",
                    class.get()
                )
            }
        />
    }
}
