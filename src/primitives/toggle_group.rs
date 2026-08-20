use crate::{primitives::roving_focus::use_roving_focus, utils::types::Orientation};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ToggleGroupType {
    /// At most one item pressed at a time.
    #[default]
    Single,
    /// Any number of items pressed at once.
    Multiple,
}

#[derive(Copy, Clone)]
pub struct ToggleGroupContext {
    pub value: RwSignal<Vec<String>>,
    pub kind: ToggleGroupType,
    pub disabled: Signal<bool>,
}

impl ToggleGroupContext {
    pub fn is_pressed(&self, value: &str) -> bool {
        self.value
            .with(|current| current.iter().any(|v| v == value))
    }

    pub fn toggle(&self, value: &str) {
        let kind = self.kind;
        self.value.update(|current| {
            let existing = current.iter().position(|v| v == value);
            match (kind, existing) {
                (ToggleGroupType::Single, Some(_)) => current.clear(),
                (ToggleGroupType::Single, None) => {
                    current.clear();
                    current.push(value.to_string());
                }
                (ToggleGroupType::Multiple, Some(index)) => {
                    current.remove(index);
                }
                (ToggleGroupType::Multiple, None) => current.push(value.to_string()),
            }
        });
    }
}

pub fn use_toggle_group() -> ToggleGroupContext {
    expect_context::<ToggleGroupContext>()
}

#[component]
pub fn ToggleGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] kind: ToggleGroupType,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Omit for an uncontrolled group that owns its own selection.
    #[prop(default = None, into)]
    value: Option<RwSignal<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let context = ToggleGroupContext {
        value,
        kind,
        disabled,
    };
    let orientation_str = orientation.as_str();

    let group_ref = AnyNodeRef::new();
    let roving = use_roving_focus(group_ref, orientation);

    // A group is one tab stop, on the pressed item when there is one.
    Effect::new(move |_| {
        value.track();
        roving.sync_tab_stop();
    });

    view! {
        <Provider value=context>
            <div
                node_ref=group_ref
                role="group"
                data-slot="toggle-group"
                data-orientation=orientation_str
                aria-orientation=orientation_str
                on:keydown=move |e: ev::KeyboardEvent| {
                    if roving.on_keydown(&e.key()) {
                        e.prevent_default();
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
pub fn ToggleGroupItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_toggle_group();

    let item = value.clone();
    let is_pressed = Memo::new(move |_| ctx.is_pressed(&item));
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    let on_click = move |_: ev::MouseEvent| {
        if !is_disabled.get() {
            ctx.toggle(&value);
        }
    };

    view! {
        <button
            type="button"
            data-slot="toggle-group-item"
            tabindex="-1"
            data-roving-item=""
            aria-pressed=move || if is_pressed.get() { "true" } else { "false" }
            data-state=move || if is_pressed.get() { "on" } else { "off" }
            disabled=is_disabled
            on:click=on_click
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
