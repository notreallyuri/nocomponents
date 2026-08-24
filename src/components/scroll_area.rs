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
    #[prop(default = false)] horizontal: bool,
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
                // The primitive's inline `scrollbar-width` covers the standard property; WebKit
                // needs a pseudo-element rule, which an inline style cannot write.
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
                    // Shown while the pointer is anywhere in the area, like an overlay bar.
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
