//! A set of radios where exactly one may be chosen.
//!
//! One tab stop: Tab reaches the checked radio, the arrows move between them, and — unlike a menu
//! — moving is choosing, as the radio pattern and a native group both have it.

use crate::{
    primitives::{
        field::{FieldControl, FieldSetControl},
        roving_focus::use_roving_focus,
    },
    utils::{
        controllable::controllable,
        form::{FormInput, on_form_reset},
        types::Orientation,
    },
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct RadioGroupContext {
    pub value: RwSignal<Option<String>>,
    pub disabled: Signal<bool>,
    /// The name every item's native radio is submitted under. Empty leaves the group out of a
    /// form's data.
    pub name: Signal<String>,
    pub required: Signal<bool>,
    pub form: Signal<String>,
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
    /// The value an uncontrolled group starts with. Ignored when `value` is given. Declared
    /// `optional, into` so that `default_value="a"` compiles: `&str` does not convert into
    /// `Option<String>`.
    #[prop(optional, into)]
    default_value: Option<String>,
    /// Called with the new value whenever the choice changes, however that came about.
    #[prop(optional, into)]
    on_value_change: Option<Callback<Option<String>>>,
    /// The name the choice is submitted under: the checked item's `value`, through a native radio
    /// each item keeps beside its button. Empty leaves it out of a form's data.
    #[prop(optional, into)]
    name: Signal<String>,
    /// The browser refuses to submit the form while no item is chosen.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// The `id` of a `<form>` the group belongs to when it is not inside one. Empty means the one
    /// around it.
    #[prop(optional, into)]
    form: Signal<String>,
    children: Children,
) -> impl IntoView {
    let value = controllable(value, default_value, on_value_change);
    let initial = value.get_untracked();
    // A radiogroup is what a `<fieldset>`'s legend names: there is no one control for `for` to
    // point at, so the group points back. Outside a fieldset these read empty.
    let group = FieldSetControl::new(disabled);
    let context = RadioGroupContext {
        value,
        disabled: group.disabled,
        name,
        required,
        form,
    };

    let group_ref = AnyNodeRef::new();
    on_form_reset(group_ref, move || value.set(initial.clone()));
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
                aria-labelledby=move || group.labelled_by.get()
                aria-describedby=move || group.described_by.get()
                aria-invalid=move || group.invalid.get().then_some("true")
                data-disabled=move || group.disabled.get().then_some("true")
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

    // A radio inside its own `Field` wears the id that field's label points `for` at — the same
    // wiring the checkbox and the switch have. A radio *group* is labelled by a fieldset instead,
    // which is a different question with a different answer: see `FieldSetControl`.
    let field = FieldControl::new(id, disabled);

    let item = value.clone();
    let submitted = value.clone();
    let node_ref = AnyNodeRef::new();
    let is_checked = Memo::new(move |_| ctx.is_checked(&item));
    let is_disabled = Signal::derive(move || field.disabled.get() || ctx.disabled.get());

    view! {
        <button
            node_ref=node_ref
            type="button"
            id=move || field.id.get()
            role="radio"
            tabindex="-1"
            data-roving-item=""
            data-slot="radio-group-item"
            aria-checked=move || if is_checked.get() { "true" } else { "false" }
            aria-describedby=move || field.described_by.get()
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
        <FormInput
            control=node_ref
            input_type="radio"
            name=ctx.name
            value=submitted
            form=ctx.form
            checked=is_checked
            required=ctx.required
            disabled=is_disabled
        />
    }
}
