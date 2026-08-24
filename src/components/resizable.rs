use crate::{
    cn,
    primitives::resizable::{ResizableGroupRoot, ResizableHandleRoot, ResizablePanelRoot},
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn ResizableGroup(
    #[prop(default = Orientation::Horizontal)] direction: Orientation,
    #[prop(optional, into)] sizes: RwSignal<Vec<f64>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ResizableGroupRoot
            direction=direction
            sizes=sizes
            class=move || {
                cn!(
                    "flex h-full w-full data-[orientation=vertical]:flex-col",
                    class.get()
                )
            }
        >
            {children()}
        </ResizableGroupRoot>
    }
}

#[component]
pub fn ResizablePanel(
    #[prop(default = None, into)] default_size: Option<f64>,
    #[prop(default = 0.0)] min_size: f64,
    #[prop(default = 100.0)] max_size: f64,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <ResizablePanelRoot
            default_size=default_size
            min_size=min_size
            max_size=max_size
            class=class
        >
            {children()}
        </ResizablePanelRoot>
    }
}

#[component]
pub fn ResizableHandle(
    index: usize,
    #[prop(default = false)] with_handle: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ResizableHandleRoot
            index=index
            disabled=disabled
            class=move || {
                cn!(
                    "group/resize-handle relative flex w-px items-center justify-center bg-border outline-none select-none",
                    // The divider is a hairline, but what you can grab is wider than what you can
                    // see: the pseudo-element widens the target without moving the line.
                    "after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2",
                    "focus-visible:ring-3 focus-visible:ring-ring/50 data-dragging:bg-ring",
                    "data-[orientation=vertical]:h-px data-[orientation=vertical]:w-full data-[orientation=vertical]:after:inset-x-0 data-[orientation=vertical]:after:top-1/2 data-[orientation=vertical]:after:h-1 data-[orientation=vertical]:after:w-full data-[orientation=vertical]:after:-translate-y-1/2 data-[orientation=vertical]:after:translate-x-0",
                    "cursor-col-resize data-[orientation=vertical]:cursor-row-resize data-disabled:cursor-default data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            {with_handle
                .then(|| {
                    view! {
                        // The variant is on the handle, so the grip reads it through the group rather
                        // than looking for an orientation it does not carry itself.
                        <div class="z-10 flex h-4 w-3 items-center justify-center rounded-xs border bg-border group-data-[orientation=vertical]/resize-handle:h-3 group-data-[orientation=vertical]/resize-handle:w-4">
                            <div class="flex gap-0.5 group-data-[orientation=vertical]/resize-handle:flex-col">
                                <div class="size-0.5 rounded-full bg-foreground/50" />
                                <div class="size-0.5 rounded-full bg-foreground/50" />
                            </div>
                        </div>
                    }
                })}
        </ResizableHandleRoot>
    }
}
