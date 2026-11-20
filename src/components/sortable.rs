use crate::{
    cn,
    icons::grip::Grip,
    primitives::sortable::{
        SortableHandleRoot, SortableItemRoot, SortableRoot,
        window::{SortableWindowHandleRoot, SortableWindowRoot},
    },
    utils::{
        controllable::forward,
        icon::IconSlot,
        types::{AsClass, Orientation},
    },
};
use leptos::prelude::*;
use std::collections::HashSet;

#[allow(unused_imports)]
pub use crate::primitives::sortable::{
    SortableEvent, SortableMove, SortableProgress, SortableStatus, apply_move,
    default_announcement, default_progress, window::SortableWindowMove,
};

const PILE: &str = "data-[count]:rounded-md data-[count]:border data-[count]:border-muted-foreground/40 data-[count]:bg-card data-[count]:shadow-[3px_3px_0_0_var(--card),3px_3px_0_1px_color-mix(in_oklab,var(--muted-foreground)_40%,transparent),6px_6px_0_0_var(--card),6px_6px_0_1px_color-mix(in_oklab,var(--muted-foreground)_40%,transparent)] data-[count]:data-[following=true]:shadow-[3px_3px_0_0_var(--card),3px_3px_0_1px_color-mix(in_oklab,var(--muted-foreground)_40%,transparent),6px_6px_0_0_var(--card),6px_6px_0_1px_color-mix(in_oklab,var(--muted-foreground)_40%,transparent),0_12px_20px_-6px_rgb(0_0_0/0.35)]";

const BADGE: &str = "data-[count]:after:absolute data-[count]:after:flex data-[count]:after:size-5 data-[count]:after:items-center data-[count]:after:justify-center data-[count]:after:rounded-full data-[count]:after:bg-primary data-[count]:after:text-xs data-[count]:after:font-medium data-[count]:after:text-primary-foreground data-[count]:after:content-[attr(data-count)]";

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SortableItemVariant {
    #[default]
    Card,
    Plain,
}

impl AsClass for SortableItemVariant {
    fn as_class(&self) -> &'static str {
        match self {
            SortableItemVariant::Card => {
                "rounded-md border bg-card p-2.5 shadow-sm data-[selected=true]:not-data-[count]:border-primary/50 data-[selected=true]:not-data-[count]:bg-primary/5 data-[placeholder=true]:border-primary"
            }
            SortableItemVariant::Plain => {
                "rounded-md p-2 hover:bg-accent hover:text-accent-foreground data-[selected=true]:not-data-[count]:bg-accent data-[following=true]:not-data-[count]:bg-accent"
            }
        }
    }
}

#[component]
pub fn Sortable(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = Orientation::Vertical.into(), into)] orientation: Signal<Orientation>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] selected: Signal<HashSet<String>>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] instructions: Signal<String>,
    #[prop(default = None, into)] announce: Option<Callback<SortableEvent, String>>,
    #[prop(default = None, into)] progress: Option<Callback<SortableProgress, String>>,
    #[prop(into)] on_move: Callback<SortableMove>,
    children: Children,
) -> impl IntoView {
    view! {
        <SortableRoot
            orientation=orientation
            disabled=disabled
            selected=selected
            label=label
            instructions=instructions
            announce=announce
            progress=progress
            on_move=on_move
            class=move || {
                cn!(
                    "group/sortable flex gap-2 data-[orientation=horizontal]:flex-row data-[orientation=vertical]:flex-col",
                    class.get(),
                )
            }
        >
            {children()}
        </SortableRoot>
    }
}

#[component]
pub fn SortableItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] variant: Signal<SortableItemVariant>,
    children: Children,
) -> impl IntoView {
    view! {
        <SortableItemRoot
            value=value
            disabled=disabled
            label=label
            class=move || {
                cn!(
                    "relative flex items-center gap-2 text-sm outline-none select-none",
                    variant.get().as_class(),
                    "group-data-[dragging=true]/sortable:transition-transform group-data-[dragging=true]/sortable:duration-150 motion-reduce:transition-none",
                    "focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:border-ring",
                    "data-[disabled=true]:opacity-60",
                    "data-[dragging=true]:z-40 data-[dragging=true]:not-data-[count]:opacity-90",
                    "data-[following=true]:cursor-grabbing data-[following=true]:transition-none data-[following=true]:shadow-lg data-[following=true]:ring-2 data-[following=true]:ring-primary/40",
                    "data-[anchor=true]:z-50",
                    PILE,
                    BADGE,
                    "data-[count]:after:-top-2 data-[count]:after:-right-2 data-[count]:scroll-m-3",
                    "data-[placeholder=true]:ring-2 data-[placeholder=true]:ring-primary/40",
                    class.get(),
                )
            }
        >
            {children()}
        </SortableItemRoot>
    }
}

#[component]
pub fn SortableHandle(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <SortableHandleRoot
            label=label
            class=move || {
                cn!(
                    "inline-flex size-8 shrink-0 cursor-grab touch-none items-center justify-center rounded-md text-muted-foreground outline-none select-none",
                    "hover:bg-accent hover:text-accent-foreground",
                    "focus-visible:ring-[3px] focus-visible:ring-ring/50",
                    "data-[dragging=true]:cursor-grabbing data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
                    class.get(),
                )
            }
        >
            {match children {
                Some(children) => children().into_any(),
                None => {
                    view! {
                        <IconSlot
                            icon=icon
                            fallback=|| view! { <Grip /> }
                            data_slot="sortable-handle-icon"
                            class="flex size-4 items-center justify-center [&>svg]:size-full"
                        />
                    }
                        .into_any()
                }
            }}
        </SortableHandleRoot>
    }
}

#[component]
pub fn SortableWindow(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] count: Signal<usize>,
    row_height: f64,
    #[prop(default = 6)] overscan: usize,
    #[prop(into)] key: Callback<usize, String>,
    #[prop(optional, into)] item_label: Option<Callback<usize, String>>,
    #[prop(into)] row: Callback<usize, AnyView>,
    #[prop(optional, into)] row_class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] instructions: Signal<String>,
    #[prop(default = None, into)] announce: Option<Callback<SortableEvent, String>>,
    #[prop(default = None, into)] progress: Option<Callback<SortableProgress, String>>,
    #[prop(optional, into)] selected: Signal<HashSet<String>>,
    #[prop(into)] on_move: Callback<SortableWindowMove>,
    #[prop(optional, into)] on_dragging_change: Option<Callback<bool>>,
) -> impl IntoView {
    let row_classes = Memo::new(move |_| {
        cn!(
            "flex items-center gap-2 border-b bg-card px-2 outline-none select-none",
            "group-data-[dragging=true]/sortable-window:transition-transform group-data-[dragging=true]/sortable-window:duration-150 motion-reduce:transition-none",
            "focus-visible:ring-[3px] focus-visible:ring-inset focus-visible:ring-ring/50",
            "data-[disabled=true]:opacity-60",
            "data-[dragging=true]:z-40",
            "data-[following=true]:cursor-grabbing data-[following=true]:transition-none data-[following=true]:shadow-lg data-[following=true]:ring-2 data-[following=true]:ring-inset data-[following=true]:ring-primary/40",
            "data-[selected=true]:not-data-[following=true]:not-data-[count]:bg-primary/5",
            "data-[count]:right-2.5!",
            PILE,
            BADGE,
            "data-[count]:after:top-1/2 data-[count]:after:right-12 data-[count]:after:-translate-y-1/2",
            "data-[anchor=true]:z-50",
            row_class.get(),
        )
    });

    view! {
        <SortableWindowRoot
            count=count
            row_height=row_height
            overscan=overscan
            key=key
            item_label=forward(item_label)
            row=row
            disabled=disabled
            label=label
            instructions=instructions
            announce=announce
            selected=selected
            on_move=on_move
            on_dragging_change=forward(on_dragging_change)
            class=move || {
                cn!("group/sortable-window rounded-md border bg-card text-sm", class.get())
            }
            row_class=row_classes
            drop_class="mx-1 box-border rounded-md border-2 border-dashed border-primary/40 bg-[color-mix(in_oklab,var(--primary)_5%,var(--card))] transition-transform duration-150 motion-reduce:transition-none"
            progress=progress
        />
    }
}

const WINDOW_HANDLE_CLASSES: &str = "inline-flex size-8 shrink-0 cursor-grab touch-none items-center justify-center rounded-md text-muted-foreground outline-none select-none hover:bg-accent hover:text-accent-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 data-[dragging=true]:cursor-grabbing data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50";

thread_local! {
    static WINDOW_HANDLE: String = cn!(WINDOW_HANDLE_CLASSES);
}

#[component]
pub fn SortableWindowHandle(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <SortableWindowHandleRoot
            label=label
            class=move || {
                let extra = class.get();
                if extra.is_empty() {
                    WINDOW_HANDLE.with(Clone::clone)
                } else {
                    cn!(WINDOW_HANDLE_CLASSES, extra)
                }
            }
        >
            {match children {
                Some(children) => children().into_any(),
                None => {
                    view! {
                        <IconSlot
                            icon=icon
                            fallback=|| view! { <Grip /> }
                            data_slot="sortable-handle-icon"
                            class="flex size-4 items-center justify-center [&>svg]:size-full"
                        />
                    }
                        .into_any()
                }
            }}
        </SortableWindowHandleRoot>
    }
}
