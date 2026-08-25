//! Label, control, description and error, wired to each other.
//!
//! The wiring is all ids — the label's `for`, the control's `aria-describedby` — and a caller
//! should not have to invent them. The root mints them once and the parts read what they need.
//!
//! Controls opt in rather than being wrapped, since a field cannot reach into arbitrary children:
//! they build a [`FieldControl`] and wear what it resolves to, which is empty outside a field.
//!
//! The one thing the field observes rather than mints is `touched`: focus went in and came out
//! again. That is a `focusout` on the root — the event that bubbles — with the related target
//! deciding whether focus actually left, since moving between two controls inside one field is
//! not leaving it.

use crate::utils::{next_id, types::Orientation};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::Node;

#[derive(Copy, Clone)]
pub struct FieldContext {
    /// Minted by the root, worn by the control, pointed at by the label's `for`.
    pub control_id: RwSignal<String>,
    /// Empty until the matching part mounts and registers itself, the way `dialog.rs` does it.
    pub description_id: RwSignal<String>,
    pub error_id: RwSignal<String>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
    /// Whether focus has been in this field and left again — what a form means by "touched", and
    /// what tells "not filled in yet" from "left empty". Set by the root, and left alone
    /// afterwards: a caller resetting a form sets it back.
    pub touched: RwSignal<bool>,
}

impl FieldContext {
    /// The ids the control announces: the description always, the error only while invalid.
    pub fn described_by(&self) -> Option<String> {
        let mut ids = Vec::new();

        self.description_id.with(|id| {
            if !id.is_empty() {
                ids.push(id.clone());
            }
        });

        if self.invalid.get() {
            self.error_id.with(|id| {
                if !id.is_empty() {
                    ids.push(id.clone());
                }
            });
        }

        (!ids.is_empty()).then(|| ids.join(" "))
    }
}

pub fn use_field() -> FieldContext {
    expect_context::<FieldContext>()
}

/// The form for controls that may or may not find themselves inside a field.
pub fn use_field_optional() -> Option<FieldContext> {
    use_context::<FieldContext>()
}

/// The attributes a control wears inside a field, resolved against the caller's own — a supplied
/// `id` wins. Outside a field every signal reads empty, so these can be rendered unconditionally.
#[derive(Copy, Clone)]
pub struct FieldControl {
    pub id: Signal<String>,
    pub described_by: Signal<Option<String>>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
    /// Whether focus has been in the field around this control and left again. Always false
    /// outside a field, the way the rest of these read empty.
    pub touched: Signal<bool>,
}

impl FieldControl {
    pub fn new(id: Signal<String>, disabled: Signal<bool>) -> Self {
        let ctx = use_field_optional();

        Self {
            id: Signal::derive(move || {
                let own = id.get();
                match (own.is_empty(), ctx) {
                    (true, Some(ctx)) => ctx.control_id.get(),
                    _ => own,
                }
            }),
            described_by: Signal::derive(move || ctx.and_then(|ctx| ctx.described_by())),
            invalid: Signal::derive(move || ctx.is_some_and(|ctx| ctx.invalid.get())),
            disabled: Signal::derive(move || {
                disabled.get() || ctx.is_some_and(|ctx| ctx.disabled.get())
            }),
            touched: Signal::derive(move || ctx.is_some_and(|ctx| ctx.touched.get())),
        }
    }
}

/// The `disabled` a part wears inside a field or a fieldset: its own, or the one around it.
///
/// [`FieldControl`] is the whole set — the id the label points at, `aria-describedby`,
/// `aria-invalid` — and is what a *control* wants. Not everything inside a field is one: a button
/// is not the thing the label names and is not the thing that goes invalid, and the only state it
/// has any business inheriting is being switched off with the rest of the field.
///
/// The fieldset is in here too even though `<fieldset disabled>` already stops the controls
/// inside it: a part that renders as something else — a `<button>` that is really an `<a>` — is
/// not one of the controls the browser disables, and has to be told.
pub fn field_disabled(disabled: Signal<bool>) -> Signal<bool> {
    let field = use_field_optional();
    let set = use_field_set_optional();

    Signal::derive(move || {
        disabled.get()
            || field.is_some_and(|field| field.disabled.get())
            || set.is_some_and(|set| set.disabled.get())
    })
}

/// The [`FieldControl`] for a trigger that already has an id of its own — a select's, a
/// combobox's. Two of them cannot be wired the way an `<input>` is.
///
/// A field points its label's `for` at the id it minted, and a floating trigger already wears one
/// from its own root, which the label would then be naming past. Rather than the two fighting over
/// the attribute, the field is told to use the trigger's: the label ends up naming the thing the
/// pointer actually lands on. `aria-describedby` and `aria-invalid` are written onto the node for
/// the reason [`crate::primitives::floating`]'s own ARIA is — the trigger is a `<button>` in one
/// place and whatever the caller rendered in another, and the node ref is all they have in common.
///
/// Outside a field this is [`FieldControl::new`] and two effects that do nothing.
pub fn field_control_for_trigger(
    trigger_id: RwSignal<String>,
    trigger_ref: AnyNodeRef,
    disabled: Signal<bool>,
) -> FieldControl {
    let field = FieldControl::new(Signal::derive(String::new), disabled);

    if let Some(control) = use_field_optional() {
        Effect::new(move |_| {
            let trigger_id = trigger_id.get();
            if !trigger_id.is_empty() {
                control.control_id.set(trigger_id);
            }
        });
    }

    Effect::new(move |_| {
        let Some(trigger) = trigger_ref.get() else {
            return;
        };

        match field.described_by.get() {
            Some(ids) => {
                let _ = trigger.set_attribute("aria-describedby", &ids);
            }
            None => {
                let _ = trigger.remove_attribute("aria-describedby");
            }
        }

        if field.invalid.get() {
            let _ = trigger.set_attribute("aria-invalid", "true");
        } else {
            let _ = trigger.remove_attribute("aria-invalid");
        }
    });

    field
}

#[component]
pub fn FieldRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Label above control by default; `Horizontal` puts them on one line, control first.
    #[prop(default = Orientation::Vertical)]
    orientation: Orientation,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = FieldContext {
        control_id: RwSignal::new(next_id("field-control")),
        description_id: RwSignal::new(String::new()),
        error_id: RwSignal::new(String::new()),
        invalid,
        disabled,
        touched: RwSignal::new(false),
    };

    let orientation_str = orientation.as_str();
    let field_ref = AnyNodeRef::new();

    // `focusout` rather than the control's own `blur`, for two reasons: `blur` does not bubble, so
    // the field would have to reach into children it knows nothing about, and a styled control may
    // wrap itself in a box — the native select does, for its chevron — which is where a handler
    // written at the call site lands and never fires. One listener up here catches all of them.
    let on_focusout = move |e: ev::FocusEvent| {
        // Moving from a control to a button beside it is not leaving the field, and a field whose
        // error appears while you are still in it is the thing `touched` exists to avoid.
        let staying = e
            .related_target()
            .and_then(|target| target.dyn_into::<Node>().ok())
            .zip(field_ref.get())
            .is_some_and(|(target, field)| field.contains(Some(&target)));

        if !staying {
            context.touched.set(true);
        }
    };

    view! {
        <Provider value=context>
            <div
                node_ref=field_ref
                role="group"
                data-slot="field"
                data-orientation=orientation_str
                data-invalid=move || invalid.get().then_some("true")
                data-disabled=move || disabled.get().then_some("true")
                data-touched=move || context.touched.get().then_some("true")
                on:focusout=on_focusout
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// A set of controls under one label — a radio group, a row of checkboxes, three inputs that are
/// one address.
///
/// The difference from [`FieldContext`] is the whole reason this exists: a field labels *one*
/// control, and `for` can only point at one. A group is labelled by pointing the group's own
/// element at the legend with `aria-labelledby`, which is what [`FieldSetControl`] resolves.
#[derive(Copy, Clone)]
pub struct FieldSetContext {
    /// Minted by the root, worn by the legend, pointed at by every group inside.
    pub legend_id: RwSignal<String>,
    /// Empty until a description or an error mounts under this fieldset rather than under a
    /// field, exactly as [`FieldContext`] fills its own.
    pub description_id: RwSignal<String>,
    pub error_id: RwSignal<String>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl FieldSetContext {
    /// The ids the group announces: the description always, the error only while invalid.
    pub fn described_by(&self) -> Option<String> {
        let mut ids = Vec::new();

        self.description_id.with(|id| {
            if !id.is_empty() {
                ids.push(id.clone());
            }
        });

        if self.invalid.get() {
            self.error_id.with(|id| {
                if !id.is_empty() {
                    ids.push(id.clone());
                }
            });
        }

        (!ids.is_empty()).then(|| ids.join(" "))
    }
}

pub fn use_field_set() -> FieldSetContext {
    expect_context::<FieldSetContext>()
}

/// The form for groups that may or may not find themselves inside a fieldset.
pub fn use_field_set_optional() -> Option<FieldSetContext> {
    use_context::<FieldSetContext>()
}

/// The attributes a *group* wears inside a fieldset — the counterpart to [`FieldControl`], which
/// is for a single control. Outside a fieldset every signal reads empty, so a group can render
/// these unconditionally.
///
/// A group carries no `id` of its own here: nothing points `for` at a radiogroup, so the wiring
/// runs the other way and the group names the legend.
#[derive(Copy, Clone)]
pub struct FieldSetControl {
    pub labelled_by: Signal<Option<String>>,
    pub described_by: Signal<Option<String>>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl FieldSetControl {
    pub fn new(disabled: Signal<bool>) -> Self {
        let ctx = use_field_set_optional();

        Self {
            labelled_by: Signal::derive(move || {
                ctx.and_then(|ctx| {
                    ctx.legend_id
                        .with(|id| (!id.is_empty()).then(|| id.clone()))
                })
            }),
            described_by: Signal::derive(move || ctx.and_then(|ctx| ctx.described_by())),
            invalid: Signal::derive(move || ctx.is_some_and(|ctx| ctx.invalid.get())),
            disabled: Signal::derive(move || {
                disabled.get() || ctx.is_some_and(|ctx| ctx.disabled.get())
            }),
        }
    }
}

/// A real `<fieldset>`, because `disabled` on one disables every control inside it without each
/// one having to be told — the single behaviour a `<div role="group">` could not have borrowed.
///
/// The legend is *not* rendered here: [`FieldLegendRoot`] is a child like every other part, so a
/// caller can put a heading, a badge or nothing at all in it.
#[component]
pub fn FieldSetRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = FieldSetContext {
        legend_id: RwSignal::new(next_id("field-legend")),
        description_id: RwSignal::new(String::new()),
        error_id: RwSignal::new(String::new()),
        invalid,
        disabled,
    };

    view! {
        <Provider value=context>
            <fieldset
                data-slot="field-set"
                data-invalid=move || invalid.get().then_some("true")
                data-disabled=move || disabled.get().then_some("true")
                disabled=move || disabled.get()
                class=class
            >
                {children()}
            </fieldset>
        </Provider>
    }
}

/// What the fieldset is called. A `<legend>` rather than a `<label>`: it names the set, and there
/// is no one control for it to point at.
#[component]
pub fn FieldLegendRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_field_set();

    view! {
        <legend
            id=move || ctx.legend_id.get()
            data-slot="field-legend"
            data-invalid=move || ctx.invalid.get().then_some("true")
            data-disabled=move || ctx.disabled.get().then_some("true")
            class=class
        >
            {children()}
        </legend>
    }
}

/// Registers an id on the nearest field — or on the fieldset, when a description belongs to a
/// whole group rather than to one control — so that what is described can point
/// `aria-describedby` at this element. Returns the id for the element to render.
pub fn use_field_part_id(part: FieldPart) -> String {
    let id = next_id(match part {
        FieldPart::Description => "field-description",
        FieldPart::Error => "field-error",
    });

    match (use_field_optional(), use_field_set_optional()) {
        (Some(field), _) => match part {
            FieldPart::Description => field.description_id.set(id.clone()),
            FieldPart::Error => field.error_id.set(id.clone()),
        },
        (None, Some(set)) => match part {
            FieldPart::Description => set.description_id.set(id.clone()),
            FieldPart::Error => set.error_id.set(id.clone()),
        },
        // Neither: the same panic a part has always raised outside a field, since an id nothing
        // reads would be worse than being told where the part belongs.
        (None, None) => {
            use_field();
        }
    }

    id
}

/// The describing parts of a field, for [`use_field_part_id`].
#[derive(Clone, Copy)]
pub enum FieldPart {
    Description,
    Error,
}

#[component]
pub fn FieldLabelRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_field();

    view! {
        <label
            data-slot="field-label"
            r#for=move || ctx.control_id.get()
            data-invalid=move || ctx.invalid.get().then_some("true")
            data-disabled=move || ctx.disabled.get().then_some("true")
            class=class
        >
            {children()}
        </label>
    }
}

#[component]
pub fn FieldDescriptionRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let id = use_field_part_id(FieldPart::Description);

    view! {
        <p id=id data-slot="field-description" class=class>
            {children()}
        </p>
    }
}

/// Rendered only while the field is invalid, which is why it may carry `role="alert"`. Its id is
/// registered outside the `Show`, so it stays the same across every toggle.
#[component]
pub fn FieldErrorRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    // Whichever of the two this error belongs to is what decides it is showing — a set of
    // checkboxes has one error between them, not one each.
    let field = use_field_optional();
    let set = use_field_set_optional();
    let invalid = Signal::derive(move || match (field, set) {
        (Some(field), _) => field.invalid.get(),
        (None, Some(set)) => set.invalid.get(),
        (None, None) => false,
    });
    let id = use_field_part_id(FieldPart::Error);
    let children = StoredValue::new(children);

    view! {
        <Show when=move || invalid.get()>
            <p id=id.clone() role="alert" data-slot="field-error" class=class>
                {children.with_value(|c| c())}
            </p>
        </Show>
    }
}
