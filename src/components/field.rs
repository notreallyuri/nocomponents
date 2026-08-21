//! A labelled control with its description and error.
//!
//! Styling only; the ids and the wiring are `src/primitives/field.rs`'s. What is decided here is
//! the layout of the two orientations.

use crate::{
    cn,
    primitives::field::{FieldDescriptionRoot, FieldErrorRoot, FieldLabelRoot, FieldRoot},
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn Field(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <FieldRoot
            orientation=orientation
            invalid=invalid
            disabled=disabled
            class=move || {
                cn!(
                    // `group` so `Label` keeps working inside a field and the parts can key off
                    // the field's disabled state.
                    "group flex w-full gap-2 data-[orientation=vertical]:flex-col data-[orientation=horizontal]:flex-row data-[orientation=horizontal]:items-center data-[orientation=horizontal]:gap-3 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-60",
                    class.get()
                )
            }
        >
            {children()}
        </FieldRoot>
    }
}

#[component]
pub fn FieldLabel(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <FieldLabelRoot class=move || {
            cn!(
                "flex items-center gap-2 text-sm leading-none font-medium select-none data-[invalid=true]:text-destructive data-[disabled=true]:opacity-50",
                class.get()
            )
        }>{children()}</FieldLabelRoot>
    }
}

#[component]
pub fn FieldDescription(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <FieldDescriptionRoot class=move || {
            cn!("text-sm leading-normal text-muted-foreground", class.get())
        }>{children()}</FieldDescriptionRoot>
    }
}

/// Shown only while the field is invalid; see `FieldErrorRoot`.
#[component]
pub fn FieldError(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <FieldErrorRoot class=move || {
            cn!("text-sm leading-normal font-medium text-destructive", class.get())
        }>{children()}</FieldErrorRoot>
    }
}

/// Stacks fields into a form. Style-only and contextless: unrelated fields should not pretend to
/// be a `<fieldset>`.
#[component]
pub fn FieldGroup(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-slot="field-group" class=move || cn!("flex w-full flex-col gap-6", class.get())>
            {children()}
        </div>
    }
}
