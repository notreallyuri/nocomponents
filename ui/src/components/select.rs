use crate::{
    cn,
    primitives::select::{use_select, SelectContentRoot, SelectItemRoot, SelectRoot},
};
use leptos::prelude::*;

#[component]
pub fn Select(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <SelectRoot value=value class=move || cn!("relative w-full", class.get())>
            {children()}
        </SelectRoot>
    }
}

#[component]
pub fn SelectTrigger(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    // Funnel directly to our styled HTML element
    let ctx = use_select();

    view! {
        <button
            node_ref=ctx.trigger_ref
            type="button"
            on:click=move |_| ctx.toggle()
            class=move || {
                cn!(
                    "flex h-8 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                class.get()
                )
            }
        >
            {children()}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-4 w-4 opacity-50"
            >
                <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
        </button>
    }
}

#[component]
pub fn SelectValue(#[prop(into)] placeholder: String) -> impl IntoView {
    let ctx = use_select();

    view! {
        <span class="truncate">
            {move || ctx.value.get().unwrap_or_else(|| placeholder.clone())}
        </span>
    }
}

#[component]
pub fn SelectContent(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <SelectContentRoot class=move || {
            cn!(
                "z-50 overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95 duration-200",
                    "data-[side=top]:origin-bottom data-[side=bottom]:origin-top",
                    class.get()
            )
        }>
            <div class="p-1 w-full max-h-96 overflow-y-auto">{children()}</div>
        </SelectContentRoot>
    }
}

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <SelectItemRoot
            value=value
            class=move || {
                cn!(
                    "group relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none hover:bg-accent hover:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50",
                    class.get()
                )
            }
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4 hidden group-data-[state=checked]:block"
                >
                    <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
            </span>
            {children()}
        </SelectItemRoot>
    }
}
