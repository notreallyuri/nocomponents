use crate::{
    cn,
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::{check::Check, chevron::ChevronDown},
    primitives::combobox::{
        ComboboxContentRoot, ComboboxItemRoot, ComboboxPortalRoot, ComboboxRoot,
        ComboboxTriggerRoot, use_combobox,
    },
    utils::{controllable::forward, icon::IconSlot},
};
use leptos::{either::Either, prelude::*};

#[allow(unused_imports)]
pub use crate::utils::types::{Align, Side, SideOffset, TriggerRender};

#[component]
pub fn Combobox(
    #[prop(default = None, into)] value: Option<RwSignal<Option<String>>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] display_value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let value = Some(value.unwrap_or_else(|| RwSignal::new(default_value)));

    view! {
        <ComboboxRoot
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            value=value
            on_value_change=forward(on_value_change)
            display_value=display_value
            name=name
            form=form
            class=move || cn!("relative inline-block text-left", class.get())
        >
            {children()}
        </ComboboxRoot>
    }
}

#[component]
pub fn ComboboxTrigger(
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(default = ButtonVariant::Outline.into(), into)] variant: Signal<ButtonVariant>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let icon = StoredValue::new(icon);

    let styled = Callback::new(move |trigger: TriggerRender| {
        view! {
            <Button
                size=size
                variant=variant
                node_ref=trigger.node_ref
                on_click=trigger.on_click
                disabled=trigger.disabled
                class=move || cn!("w-56 justify-between font-normal", class.get())
            >
                {move || {
                    children
                        .with_value(|children| match children {
                            Some(children) => Either::Left(children()),
                            None => Either::Right(view! { <ComboboxValue /> }),
                        })
                }}
                <IconSlot
                    icon=icon.get_value()
                    fallback=|| view! { <ChevronDown /> }
                    data_slot="combobox-icon"
                    class="ml-2 flex size-4 shrink-0 items-center justify-center text-muted-foreground [&>svg]:size-full"
                />
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
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    #[prop(into, default = true.into())] should_filter: Signal<bool>,
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
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(into, default = true.into())] deselectable: Signal<bool>,
    #[prop(default = None, into)] selected: Option<Signal<bool>>,
    #[prop(into, default = true.into())] close_on_select: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ComboboxItemRoot
            value=value
            label=label
            keywords=keywords
            disabled=disabled
            on_select=forward(on_select)
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
                <IconSlot
                    icon=icon.clone()
                    fallback=|| view! { <Check /> }
                    data_slot="combobox-item-indicator"
                    class="hidden size-4 shrink-0 items-center justify-center group-data-[selected]/combobox-item:flex [&>svg]:size-full"
                />
            </span>
            {children()}
        </ComboboxItemRoot>
    }
}
