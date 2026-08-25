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
            ArrowOpen, FloatingContext, FloatingRoot, FloatingTrigger, TriggerAria, TriggerRender,
        },
    },
    utils::{
        get_placement,
        types::{Align, Side, SideOffset},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{either::Either, portal::Portal, prelude::*, wasm_bindgen::JsCast};
use send_wrapper::SendWrapper;
use web_sys::HtmlElement;

pub type ComboboxContext = FloatingContext;

pub fn use_combobox() -> ComboboxContext {
    expect_context::<ComboboxContext>()
}

#[component]
pub fn ComboboxRoot(
    /// The chosen value.
    #[prop(optional, into)]
    value: RwSignal<Option<String>>,
    /// What the trigger shows for that value, when the two differ. An item sets it as it is
    /// chosen; preset it alongside `value` for a selection that arrives from outside, since the
    /// items that know the labels do not exist until the list first opens.
    #[prop(optional, into)]
    display_value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = FloatingContext {
        value,
        display_value,
        ..Default::default()
    };

    view! {
        <FloatingRoot
            class=class
            context=ctx
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
        <Portal>
            <Show when=move || {
                ctx.is_mounted.get()
            }>{children.with_value(|children| children())}</Show>
        </Portal>
    }
}

#[component]
pub fn ComboboxContentRoot(
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(optional)] side_offset: SideOffset,
    /// Off when the list arrives already narrowed; forwarded to the command root.
    #[prop(default = true)]
    should_filter: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_combobox();
    let floating_ref = ctx.content_ref;

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(side_offset.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(get_placement(side, align))
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
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
    #[prop(default = None, into)]
    on_select: Option<Callback<String>>,
    /// Whether choosing the item again clears the selection, as a filter chip does.
    #[prop(default = true)]
    deselectable: bool,
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
    #[prop(default = true)]
    close_on_select: bool,
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
            if deselectable && is_selected.get_untracked() {
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

        if close_on_select {
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
