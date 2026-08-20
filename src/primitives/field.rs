//! Label, control, description and error, wired to each other.
//!
//! The point of a field is the wiring nobody writes by hand: the label's `for` has to reach the
//! control, the control's `aria-describedby` has to name the description *and* the error, and the
//! error should only be announced while the field is actually invalid. All of that is ids, and ids
//! are exactly what a caller should not have to invent. The root mints them once and publishes
//! them on a context; the parts read the one they need.
//!
//! Controls opt in rather than being wrapped. A field cannot reach into arbitrary children to add
//! attributes, so `Input`, `TextareaRoot` and `NativeSelectRoot` build a [`FieldControl`] and wear
//! whatever it resolves to. Any other control can join in the same one line, and one written
//! outside a field is unaffected — every signal reads empty when there is no context to find.

use crate::utils::{next_id, types::Orientation};
use leptos::{context::Provider, prelude::*};

#[derive(Copy, Clone)]
pub struct FieldContext {
    /// Minted by the root, worn by the control, pointed at by the label's `for`.
    pub control_id: RwSignal<String>,
    /// Empty until the matching part mounts and registers itself, the way `dialog.rs` does it.
    pub description_id: RwSignal<String>,
    pub error_id: RwSignal<String>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl FieldContext {
    /// The ids the control should announce: the description always, the error only while the field
    /// is invalid — an error nobody can see should not be read out either.
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

/// The attributes a control wears inside a field, already resolved against the caller's own.
///
/// A caller-supplied `id` wins: the field's is a fallback for the common case where nobody wanted
/// to name the control. Outside a field every signal reads empty, so a control can render these
/// unconditionally.
#[derive(Copy, Clone)]
pub struct FieldControl {
    pub id: Signal<String>,
    pub described_by: Signal<Option<String>>,
    pub invalid: Signal<bool>,
    pub disabled: Signal<bool>,
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
        }
    }
}

#[component]
pub fn FieldRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Label above control by default. `Horizontal` puts them on one line, for the checkbox and
    /// switch rows where the control comes first and the label sits beside it.
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
    };

    let orientation_str = orientation.as_str();

    view! {
        <Provider value=context>
            <div
                role="group"
                data-slot="field"
                data-orientation=orientation_str
                data-invalid=move || invalid.get().then_some("true")
                data-disabled=move || disabled.get().then_some("true")
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Registers an id on the context so the control can point `aria-describedby` at this element, and
/// returns it for the element to render.
pub fn use_field_part_id(part: FieldPart) -> String {
    let ctx = use_field();
    let id = next_id(match part {
        FieldPart::Description => "field-description",
        FieldPart::Error => "field-error",
    });

    match part {
        FieldPart::Description => ctx.description_id.set(id.clone()),
        FieldPart::Error => ctx.error_id.set(id.clone()),
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

/// Rendered only while the field is invalid, which is also why it may carry `role="alert"` — the
/// element appearing *is* the event worth announcing.
///
/// The id is registered in the component body rather than inside the `Show`, so it stays the same
/// across every toggle instead of being re-minted each time the error comes back.
#[component]
pub fn FieldErrorRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_field();
    let id = use_field_part_id(FieldPart::Error);
    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.invalid.get()>
            <p id=id.clone() role="alert" data-slot="field-error" class=class>
                {children.with_value(|c| c())}
            </p>
        </Show>
    }
}
