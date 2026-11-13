use crate::{
    cn,
    components::tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    icons::ellipsis::Ellipsis,
    primitives::floating_menu::{FloatingMenuHandleRoot, FloatingMenuRoot, use_floating_menu},
    utils::{icon::IconSlot, types::Side},
};
use leptos::{either::Either, ev, prelude::*};

#[allow(unused_imports)]
pub use crate::utils::types::Orientation;

#[component]
pub fn FloatingMenu(
    #[prop(optional)] position: Option<RwSignal<(f64, f64)>>,
    #[prop(default = (24.0, 24.0))] default_position: (f64, f64),
    #[prop(optional, into)] orientation: Signal<Orientation>,
    #[prop(optional, into)] compact: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <FloatingMenuRoot
            position=position
            default_position=default_position
            orientation=orientation
            compact=compact
            class=move || {
                cn!(
                    "z-50 flex overflow-hidden rounded-lg border bg-popover text-popover-foreground shadow-lg ring-1 ring-foreground/10",
                    "data-[orientation=vertical]:min-w-40 data-[orientation=vertical]:flex-col",
                    "data-[orientation=horizontal]:flex-row data-[orientation=horizontal]:items-stretch",
                    "data-[dragging=true]:shadow-2xl data-[dragging=true]:ring-primary/40",
                    class.get(),
                )
            }
        >
            {children()}
        </FloatingMenuRoot>
    }
}

#[component]
pub fn FloatingMenuHandle(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <FloatingMenuHandleRoot
            disabled=disabled
            class=move || {
                cn!(
                    "flex shrink-0 cursor-grab touch-none items-center justify-center gap-2 bg-muted/50 text-xs font-medium select-none",
                    "data-[orientation=vertical]:border-b data-[orientation=vertical]:px-2 data-[orientation=vertical]:py-1.5",
                    "data-[orientation=horizontal]:border-r data-[orientation=horizontal]:px-1.5 data-[orientation=horizontal]:py-2",
                    "data-[dragging=true]:cursor-grabbing",
                    class.get(),
                )
            }
        >
            <IconSlot
                icon=icon
                fallback=|| view! { <Ellipsis /> }
                data_slot="floating-menu-handle-icon"
                class="flex size-3.5 shrink-0 items-center justify-center text-muted-foreground [[data-orientation=horizontal]>&]:rotate-90 [&>svg]:size-full"
            />
            {children.map(|children| children())}
        </FloatingMenuHandleRoot>
    }
}

#[component]
pub fn FloatingMenuContent(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_floating_menu();

    view! {
        <div
            data-slot="floating-menu-content"
            data-orientation=move || ctx.orientation.get().as_str()
            class=move || {
                cn!(
                    "flex gap-1 p-1.5",
                    "data-[orientation=vertical]:flex-col",
                    "data-[orientation=horizontal]:flex-row data-[orientation=horizontal]:items-center",
                    class.get(),
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FloatingMenuItem(
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_floating_menu();
    let compact = move || ctx.compact.get();
    let children = StoredValue::new(children);

    let button = move || {
        view! {
            <button
                type="button"
                data-slot="floating-menu-item"
                data-compact=move || compact().then_some("true")
                disabled=move || disabled.get()
                aria-label=move || compact().then(|| label.get())
                on:click=move |e: ev::MouseEvent| {
                    if let Some(on_click) = on_click {
                        on_click.run(e);
                    }
                }
                class=move || {
                    cn!(
                        "inline-flex shrink-0 items-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-colors outline-none",
                        "hover:bg-accent hover:text-accent-foreground focus-visible:ring-3 focus-visible:ring-ring/50",
                        "disabled:pointer-events-none disabled:opacity-50",
                        "[&>svg]:size-4 [&>svg]:shrink-0",
                        "data-[compact=true]:size-8 data-[compact=true]:justify-center data-[compact=true]:p-0",
                        "not-data-[compact=true]:h-8 not-data-[compact=true]:justify-start not-data-[compact=true]:px-2.5",
                        class.get(),
                    )
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
                <Show when=move || !compact()>
                    <span class="truncate">{move || label.get()}</span>
                </Show>
            </button>
        }
    };

    move || match compact() {
        true => Either::Left(view! {
            <Tooltip>
                <TooltipTrigger>{button()}</TooltipTrigger>
                <TooltipContent side=Side::Right>{move || label.get()}</TooltipContent>
            </Tooltip>
        }),
        false => Either::Right(button()),
    }
}

#[component]
pub fn FloatingMenuSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_floating_menu();

    view! {
        <div
            data-slot="floating-menu-separator"
            data-orientation=move || ctx.orientation.get().as_str()
            role="none"
            class=move || {
                cn!(
                    "shrink-0 bg-border/70",
                    "data-[orientation=vertical]:h-px data-[orientation=vertical]:w-full",
                    "data-[orientation=horizontal]:my-1 data-[orientation=horizontal]:w-px data-[orientation=horizontal]:self-stretch",
                    class.get(),
                )
            }
        />
    }
}
