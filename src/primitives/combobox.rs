//! A combobox: a button that opens a searchable list and comes back with one value.
//!
//! There is no new state machine here, and that is the point — it is
//! [`crate::primitives::command`] hosted inside a [`crate::primitives::floating`] layer. The
//! floating context owns the chosen `value`, the command context owns the search and the
//! highlight, and the only piece of behaviour this file adds is the bridge between them: choosing
//! an item writes the value and closes the layer.
//!
//! Everything the list is made of — the input, the groups, the empty state — is the command
//! palette's, and is used directly.

use crate::{
    primitives::{
        command::{CommandItemRoot, CommandRoot},
        field::field_control_for_trigger,
        floating::{
            ArrowOpen, FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria,
            TriggerRender, floating_options, side_placement,
        },
    },
    utils::{
        controllable::{controllable, forward},
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{Placement, UseFloatingReturn, use_floating};
use leptos::{either::Either, prelude::*, wasm_bindgen::JsCast};
use web_sys::HtmlElement;

pub type ComboboxContext = FloatingContext;

pub fn use_combobox() -> ComboboxContext {
    expect_context::<ComboboxContext>()
}

#[component]
pub fn ComboboxRoot(
    /// The chosen value, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    value: Option<RwSignal<Option<String>>>,
    /// The value the root starts with. Ignored when `value` is given. Declared `optional, into`
    /// rather than `default = None, into` so that `default_value="apple"` compiles: `&str` does
    /// not convert into `Option<String>`.
    #[prop(optional, into)]
    default_value: Option<String>,
    /// Called with the new value whenever the choice changes, however that came about.
    #[prop(optional, into)]
    on_value_change: Option<Callback<Option<String>>>,

    /// What the trigger shows for that value, when the two differ. An item sets it as it is
    /// chosen; preset it alongside `value` for a selection that arrives from outside, since the
    /// items that know the labels do not exist until the list first opens.
    #[prop(optional, into)]
    display_value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    /// Whether the list is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the list starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the list opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = FloatingContext {
        value: controllable(value, default_value, on_value_change),
        display_value,
        ..Default::default()
    };

    view! {
        <FloatingRoot
            class=class
            context=ctx
            open=open
            default_open=default_open
            on_open_change=forward(on_open_change)
            trigger_aria=TriggerAria::Popup("listbox")
            arrow_open=ArrowOpen::Open
        >
            {children()}
        </FloatingRoot>
    }
}

#[component]
pub fn ComboboxTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_combobox();
    // The same wiring a select's trigger gets: in a field, the label names this trigger rather
    // than an id nothing wears.
    let field = field_control_for_trigger(ctx.trigger_id, ctx.trigger_ref, disabled);

    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=field.disabled
            data_slot="combobox-trigger"
            render=as_child
        >
            {match children {
                Some(children) => Either::Left(children()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn ComboboxPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_combobox();
    let children = StoredValue::new(children);

    view! {
        <FloatingPortal>
            <Show when=move || {
                ctx.is_mounted.get()
            }>{children.with_value(|children| children())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn ComboboxContentRoot(
    #[prop(optional, into)] side: Signal<Side>,
    #[prop(optional, into)] align: Signal<Align>,
    #[prop(optional, into)] side_offset: Signal<SideOffset>,
    /// Off when the list arrives already narrowed; forwarded to the command root.
    #[prop(into, default = true.into())]
    should_filter: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_combobox();
    let floating_ref = ctx.content_ref;

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        floating_options(side_placement(side, align), side_offset),
    );

    // The search box takes focus as soon as the list is up: a combobox that opens and leaves the
    // caret behind is a listbox with extra steps.
    Effect::new(move |focused: Option<bool>| {
        if focused == Some(true) || !ctx.is_open.get() || !is_positioned.get() {
            return focused.unwrap_or(false);
        }

        // Deferred a frame on purpose: the shell is `visibility: hidden` until floating-ui has
        // measured it, the style binding that lifts that is another effect in this same tick, and
        // `focus()` on a hidden element silently does nothing.
        request_animation_frame(move || {
            if let Some(content) = floating_ref.get_untracked()
                && let Ok(Some(input)) = content.query_selector("input")
                && let Ok(input) = input.dyn_into::<HtmlElement>()
            {
                let _ = input.focus();
            }
        });

        true
    });

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                let width = ctx
                    .trigger_ref
                    .get()
                    .map(|trigger| trigger.get_bounding_client_rect().width())
                    .unwrap_or(0.0);
                let position = if is_positioned.get() {
                    floating_styles.get().to_string()
                } else {
                    format!("{} visibility: hidden;", floating_styles.get())
                };
                format!("{position} min-width: {width}px;")
            }
            class="fixed z-50 pointer-events-none"
        >
            <div
                id=move || ctx.content_id.get()
                aria-labelledby=move || ctx.trigger_id.get()
                data-slot="combobox-content"
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => "left",
                    Placement::Right | Placement::RightStart | Placement::RightEnd => "right",
                    _ => "bottom",
                }
                class=class
            >
                // The list is a command palette, so the search, the filtering and the arrow keys
                // are the ones documented there.
                <CommandRoot should_filter=should_filter class="contents">
                    {children()}
                </CommandRoot>
            </div>
        </div>
    }
}

#[component]
pub fn ComboboxItemRoot(
    /// What is written into the combobox's value when this item is chosen, and what the list
    /// matches the search against.
    #[prop(into)]
    value: String,
    /// What the trigger shows once this item is chosen. Empty means the value itself.
    #[prop(optional, into)]
    label: Signal<String>,
    #[prop(optional, into)] keywords: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Run after the value has been written, with the item's value.
    #[prop(optional, into)]
    on_select: Option<Callback<String>>,
    /// Whether choosing the item again clears the selection, as a filter chip does.
    #[prop(into, default = true.into())]
    deselectable: Signal<bool>,
    /// What makes this item look chosen, when the combobox's own value is not the answer — a
    /// multi-select keeps a list of its own, and several items are ticked at once.
    ///
    /// Given, the item stops writing `value` and `display_value`: a caller that says what
    /// "chosen" means owns where it is kept, so choosing the item runs `on_select` and nothing
    /// else.
    #[prop(default = None, into)]
    selected: Option<Signal<bool>>,
    /// Whether choosing an item closes the layer. False for a list several things are picked
    /// from, where closing after the first is the bug.
    #[prop(into, default = true.into())]
    close_on_select: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_combobox();

    let item_value = StoredValue::new(value.clone());

    let is_selected = Signal::derive(move || match selected {
        Some(selected) => selected.get(),
        None => {
            item_value.with_value(|value| ctx.value.with(|chosen| chosen.as_deref() == Some(value)))
        }
    });

    let select = Callback::new(move |value: String| {
        if selected.is_none() {
            if deselectable.get_untracked() && is_selected.get_untracked() {
                ctx.value.set(None);
                ctx.display_value.set(None);
            } else {
                let label = label.get_untracked();
                ctx.display_value.set(Some(if label.is_empty() {
                    value.clone()
                } else {
                    label
                }));
                ctx.value.set(Some(value.clone()));
            }
        }

        if close_on_select.get_untracked() {
            ctx.close();
        }

        if let Some(on_select) = on_select {
            on_select.run(value);
        }
    });

    view! {
        <CommandItemRoot
            value=value
            keywords=keywords
            disabled=disabled
            selected=is_selected
            on_select=select
            class=class
        >
            {children()}
        </CommandItemRoot>
    }
}
