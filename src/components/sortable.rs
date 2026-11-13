use crate::{
    cn,
    icons::grip::Grip,
    primitives::sortable::{SortableHandleRoot, SortableItemRoot, SortableRoot},
    utils::{icon::IconSlot, types::Orientation},
};
use leptos::prelude::*;
use std::collections::HashSet;

#[allow(unused_imports)]
pub use crate::primitives::sortable::{
    SortableEvent, SortableMove, SortableStatus, apply_move, default_announcement,
};

#[component]
pub fn Sortable(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = Orientation::Vertical.into(), into)] orientation: Signal<Orientation>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] selected: Signal<HashSet<String>>,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] instructions: Signal<String>,
    #[prop(default = None, into)] announce: Option<Callback<SortableEvent, String>>,
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
    children: Children,
) -> impl IntoView {
    view! {
        <SortableItemRoot
            value=value
            disabled=disabled
            label=label
            class=move || {
                cn!(
                    "relative flex items-center gap-2 rounded-md border bg-card p-2.5 text-sm shadow-sm outline-none select-none",
                    "group-data-[dragging=true]/sortable:transition-transform group-data-[dragging=true]/sortable:duration-150 motion-reduce:transition-none",
                    "focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:border-ring",
                    "data-[selected=true]:border-primary/50 data-[selected=true]:bg-primary/5",
                    "data-[disabled=true]:opacity-60",
                    "data-[dragging=true]:z-40 data-[dragging=true]:opacity-90",
                    "data-[following=true]:cursor-grabbing data-[following=true]:transition-none data-[following=true]:shadow-lg data-[following=true]:ring-2 data-[following=true]:ring-primary/40",
                    "data-[anchor=true]:z-50",
                    "data-[count]:after:absolute data-[count]:after:-top-2 data-[count]:after:-right-2 data-[count]:after:flex data-[count]:after:size-5 data-[count]:after:items-center data-[count]:after:justify-center data-[count]:after:rounded-full data-[count]:after:bg-primary data-[count]:after:text-xs data-[count]:after:font-medium data-[count]:after:text-primary-foreground data-[count]:after:content-[attr(data-count)]",
                    "data-[placeholder=true]:border-primary data-[placeholder=true]:ring-2 data-[placeholder=true]:ring-primary/40",
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
