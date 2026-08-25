use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::{check::Check, chevron::ChevronDown},
    primitives::combobox::{
        ComboboxContentRoot, ComboboxItemRoot, ComboboxPortalRoot, ComboboxRoot,
        ComboboxTriggerRoot, use_combobox,
    },
    primitives::floating::TriggerRender,
    utils::types::{Align, Side, SideOffset},
};
use leptos::{either::Either, prelude::*};

#[component]
pub fn Combobox(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] display_value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ComboboxRoot
            value=value
            display_value=display_value
            class=move || cn!("relative inline-block text-left", class.get())
        >
            {children()}
        </ComboboxRoot>
    }
}

#[component]
pub fn ComboboxTrigger(
    #[prop(optional)] size: ButtonSize,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let styled = Callback::new(move |trigger: TriggerRender| {
        view! {
            <Button
                size=size
                variant=variant
                node_ref=trigger.node_ref
                on_click=trigger.on_click
                attr:disabled=trigger.disabled
                class=move || cn!("w-56 justify-between font-normal", class.get())
            >
                {move || {
                    children
                        .with_value(|children| match children {
                            Some(children) => Either::Left(children()),
                            None => Either::Right(view! { <ComboboxValue /> }),
                        })
                }}
                <ChevronDown class="ml-2 shrink-0 text-muted-foreground" />
            </Button>
        }
        .into_any()
    });

    view! { <ComboboxTriggerRoot disabled=disabled as_child=render.unwrap_or(styled) /> }
}

#[component]
pub fn ComboboxValue(
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_combobox();
    let chosen = Signal::derive(move || ctx.display_value.get().or_else(|| ctx.value.get()));

    view! {
        <span
            data-slot="combobox-value"
            data-placeholder=move || chosen.get().is_none().then_some("")
            class=move || { cn!("truncate data-[placeholder]:text-muted-foreground", class.get()) }
        >
            {move || {
                chosen
                    .get()
                    .unwrap_or_else(|| {
                        let placeholder = placeholder.get();
                        if placeholder.is_empty() { "Select…".to_string() } else { placeholder }
                    })
            }}
        </span>
    }
}

#[component]
pub fn ComboboxContent(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    #[prop(default = true)] should_filter: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ComboboxPortalRoot>
            <ComboboxContentRoot
                side=side
                align=align
                side_offset=side_offset
                should_filter=should_filter
                class=move || {
                    cn!(
                        "flex flex-col overflow-hidden rounded-lg bg-popover text-popover-foreground shadow-md ring-1 ring-foreground/10 outline-none animation-duration-100 pointer-events-auto data-[state=closed]:pointer-events-none",
                        "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
                        "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                        class.get()
                    )
                }
            >
                {children.with_value(|children| children())}
            </ComboboxContentRoot>
        </ComboboxPortalRoot>
    }
}

#[component]
pub fn ComboboxItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] label: Signal<String>,
    #[prop(optional, into)] keywords: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] on_select: Option<Callback<String>>,
    #[prop(default = true)] deselectable: bool,
    #[prop(default = None, into)] selected: Option<Signal<bool>>,
    #[prop(default = true)] close_on_select: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ComboboxItemRoot
            value=value
            label=label
            keywords=keywords
            disabled=disabled
            on_select=on_select
            deselectable=deselectable
            selected=selected
            close_on_select=close_on_select
            class=move || {
                cn!(
                    "group/combobox-item relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-none select-none data-[state=active]:bg-accent data-[state=active]:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:size-4 [&_svg]:shrink-0",
                    class.get()
                )
            }
        >
            <span class="absolute left-2 flex size-3.5 items-center justify-center">
                <Check class="hidden group-data-[selected]/combobox-item:block" />
            </span>
            {children()}
        </ComboboxItemRoot>
    }
}
