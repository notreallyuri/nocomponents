use crate::{
    primitives::{
        field::field_control_for_trigger,
        floating::{
            ArrowOpen, FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria,
            TriggerRender,
        },
        roving_focus::use_roving_focus,
        typeahead::Typeahead,
    },
    utils::{
        controllable::{controllable, forward},
        types::Orientation,
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{
    context::Provider, either::Either, ev, mount::mount_to, prelude::*, wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::{collections::HashMap, time::Duration};
use web_sys::HtmlElement;

pub type SelectContext = FloatingContext;

pub fn use_select() -> SelectContext {
    expect_context::<SelectContext>()
}

/// What each option is called, by value: the text a `SelectValue` shows once one is chosen.
///
/// Held by the root rather than by the items, because the items only exist while the list is
/// mounted and the trigger has to name the choice long before — and long after — that. Entries are
/// never removed; an option that unmounts is still the label of what was picked from it.
#[derive(Copy, Clone)]
pub struct SelectLabels(RwSignal<HashMap<String, String>>);

fn use_select_labels() -> SelectLabels {
    expect_context::<SelectLabels>()
}

/// The chosen option's label, or its value while no option has said what it is called, or `None`
/// when nothing is chosen.
pub fn use_select_label() -> Signal<Option<String>> {
    let ctx = use_select();
    let labels = use_select_labels();

    Signal::derive(move || {
        ctx.value.get().map(|value| {
            labels
                .0
                .with(|labels| labels.get(&value).cloned())
                .unwrap_or(value)
        })
    })
}
#[component]
pub fn SelectRoot(
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
    #[prop(optional, into)] class: Signal<String>,
    /// The name the choice is submitted under, through the hidden native `<select>`.
    #[prop(optional, into)]
    name: Signal<String>,
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
        ..Default::default()
    };

    let labels = SelectLabels(RwSignal::new(HashMap::new()));

    view! {
        <Provider value=labels>
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
                <select
                    class="hidden"
                    aria-hidden="true"
                    tabindex="-1"
                    name=move || Some(name.get()).filter(|name| !name.is_empty())
                >
                    <option value=move || ctx.value.get().unwrap_or_default() selected=true></option>
                </select>
            </FloatingRoot>
        </Provider>
    }
}

#[component]
pub fn SelectTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_select();
    let field = field_control_for_trigger(ctx.trigger_id, ctx.trigger_ref, disabled);

    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=field.disabled
            data_slot="select-trigger"
            render=as_child
        >
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn SelectPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_select();
    let labels = use_select_labels();
    let stored_children = StoredValue::new(children);
    let preloaded = StoredValue::new(false);

    // The items are not in the DOM until the list first opens, so a value chosen before that —
    // `default_value`, or a controlled signal set from outside — would be shown as itself. Once,
    // and only when it is needed, the content is built into a detached node under a throwaway
    // context so the items can say what they are called, then thrown away. The labels outlive it.
    Effect::new(move |_| {
        let Some(value) = ctx.value.get() else {
            return;
        };

        if preloaded.get_value()
            || ctx.is_mounted.get_untracked()
            || labels
                .0
                .with_untracked(|labels| labels.contains_key(&value))
        {
            return;
        }
        preloaded.set_value(true);

        let Ok(detached) = document().create_element("div") else {
            return;
        };
        let Ok(detached) = detached.dyn_into::<HtmlElement>() else {
            return;
        };

        let handle = mount_to(detached, move || {
            provide_context(SelectContext::default());
            stored_children.with_value(|children| children())
        });

        // The items register from an effect, once their nodes exist.
        let _ = set_timeout_with_handle(move || drop(handle), Duration::ZERO);
    });

    view! {
        <FloatingPortal>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn SelectContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let floating_ref = ctx.content_ref;

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(0.0))),
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
            .placement(Placement::BottomStart)
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    let listbox_ref = AnyNodeRef::new();
    let roving = use_roving_focus(listbox_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    // A select opens *on* its current value, the way a native one does: focus lands on the chosen
    // option, so the first arrow key steps away from it rather than from the top of the list.
    Effect::new(move |_| {
        if ctx.is_open.get() && is_positioned.get() {
            roving.focus_tab_stop();
        }
    });

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                let width = ctx
                    .trigger_ref
                    .get()
                    .map(|el| el.get_bounding_client_rect().width())
                    .unwrap_or(0.0);
                let mut base_style = if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                };
                base_style.push_str(&format!(" width: {}px;", width));
                base_style
            }
            class="fixed z-50 pointer-events-none"
        >
            <div
                node_ref=listbox_ref
                data-slot="select-content"
                id=move || ctx.content_id.get()
                role="listbox"
                aria-labelledby=move || ctx.trigger_id.get()
                tabindex="-1"
                on:keydown=move |e| {
                    let key = e.key();
                    if roving.on_keydown(&key) {
                        e.prevent_default();
                        return;
                    }
                    match key.as_str() {
                        // An option is a div, so Enter and Space have to be turned into the click
                        // that commits it.
                        "Enter" | " " => {
                            if let Some(active) = document().active_element()
                                && active.has_attribute("data-roving-item")
                                && let Ok(active) = active.dyn_into::<HtmlElement>()
                            {
                                e.prevent_default();
                                active.click();
                            }
                        }
                        "Tab" => ctx.close(),
                        _ => {
                            if typeahead.push(&key, listbox_ref) {
                                e.prevent_default();
                            }
                        }
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    _ => "bottom",
                }
                class=class
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn SelectItemRoot(
    #[prop(into)] value: String,
    /// What the trigger shows once this item is chosen. Empty means the item's own text.
    #[prop(optional, into)]
    label: Signal<String>,
    /// Skipped by the keyboard and refused by the pointer.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let labels = use_select_labels();
    let item_value = value.clone();
    let item_ref = AnyNodeRef::new();

    let is_selected = Signal::derive(move || ctx.value.get() == Some(item_value.clone()));

    let label_value = value.clone();
    Effect::new(move |_| {
        let explicit = label.get();
        let text = if explicit.is_empty() {
            let Some(el) = item_ref.get() else {
                return;
            };
            el.text_content().unwrap_or_default().trim().to_string()
        } else {
            explicit
        };

        if text.is_empty() {
            return;
        }

        labels.0.update(|labels| {
            if labels.get(&label_value) != Some(&text) {
                labels.insert(label_value.clone(), text);
            }
        });
    });

    let on_click = move |_: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        ctx.value.set(Some(value.clone()));
        ctx.close();
    };

    view! {
        <div
            node_ref=item_ref
            data-slot="select-item"
            role="option"
            tabindex="-1"
            data-roving-item=""
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
            class=class
        >
            {children()}
        </div>
    }
}
