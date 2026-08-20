//! A set of radios where exactly one may be chosen.
//!
//! The group is a single tab stop: Tab reaches the checked radio (or the first one, when nothing is
//! checked yet) and the arrow keys move between them. Unlike a menu, moving *is* choosing — the
//! radio pattern selects whatever focus lands on, which is what a native radio group does too.

use crate::{primitives::roving_focus::use_roving_focus, utils::types::Orientation};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct RadioGroupContext {
    pub value: RwSignal<Option<String>>,
    pub disabled: Signal<bool>,
}

impl RadioGroupContext {
    pub fn is_checked(&self, value: &str) -> bool {
        self.value.with(|current| current.as_deref() == Some(value))
    }

    pub fn select(&self, value: &str) {
        self.value.set(Some(value.to_string()));
    }
}

pub fn use_radio_group() -> RadioGroupContext {
    expect_context::<RadioGroupContext>()
}

#[component]
pub fn RadioGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Radios stack by default, the way a native group does; the arrow keys follow suit.
    #[prop(default = Orientation::Vertical)]
    orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Omit for an uncontrolled group that owns its own selection.
    #[prop(default = None, into)]
    value: Option<RwSignal<Option<String>>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_else(|| RwSignal::new(None));
    let context = RadioGroupContext { value, disabled };

    let group_ref = AnyNodeRef::new();
    let roving = use_roving_focus(group_ref, orientation);
    let orientation_str = orientation.as_str();

    // The tab stop follows the checked radio, so Tab lands on the current answer.
    Effect::new(move |_| {
        value.track();
        roving.sync_tab_stop();
    });

    view! {
        <Provider value=context>
            <div
                node_ref=group_ref
                role="radiogroup"
                data-slot="radio-group"
                data-orientation=orientation_str
                aria-orientation=orientation_str
                on:keydown=move |e: ev::KeyboardEvent| {
                    if roving.on_keydown(&e.key()) {
                        e.prevent_default();
                        // Selection follows focus: arrowing onto a radio checks it.
                        if let Some(active) = document().active_element()
                            && let Ok(active) = active.dyn_into::<HtmlElement>()
                        {
                            active.click();
                        }
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn RadioGroupItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_radio_group();

    let item = value.clone();
    let is_checked = Memo::new(move |_| ctx.is_checked(&item));
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    view! {
        <button
            type="button"
            id=id
            role="radio"
            tabindex="-1"
            data-roving-item=""
            data-slot="radio-group-item"
            aria-checked=move || if is_checked.get() { "true" } else { "false" }
            data-state=move || if is_checked.get() { "checked" } else { "unchecked" }
            disabled=is_disabled
            on:click=move |_| {
                if !is_disabled.get_untracked() {
                    ctx.select(&value);
                }
            }
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
