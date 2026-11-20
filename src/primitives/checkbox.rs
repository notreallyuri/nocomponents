use crate::{
    primitives::field::FieldControl,
    utils::{
        controllable::controllable,
        form::{FormInput, on_form_reset},
    },
};
use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

pub use nocomponents_core::checkbox::{checkbox_state, next_checked};

/// A two-state box that can also say "some of these": `indeterminate` is the caller's, drawn over
/// `checked` as `aria-checked="mixed"` and `data-state="indeterminate"`. Clicking it checks the box
/// and calls `on_checked_change(true)` even when `checked` was already true, and it is then up to
/// the caller to clear `indeterminate`. A form submits an indeterminate box as unchecked, as Radix
/// does.
#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Whether it is checked, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    checked: Option<RwSignal<bool>>,
    /// Whether it starts checked. Ignored when `checked` is given.
    #[prop(optional)]
    default_checked: bool,
    /// Called with the new value whenever it is checked or unchecked, however that came about.
    #[prop(optional, into)]
    on_checked_change: Option<Callback<bool>>,
    /// Shows "some, not all" instead of `checked`: `aria-checked="mixed"`, `data-state="indeterminate"`.
    #[prop(optional, into)]
    indeterminate: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: Signal<String>,
    /// The name it is submitted under. Empty leaves it out of a form's data.
    #[prop(optional, into)]
    name: Signal<String>,
    /// What it submits while checked. Empty is `"on"`, as a native checkbox has it.
    #[prop(optional, into)]
    value: Signal<String>,
    /// The browser refuses to submit the form while this is unchecked.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.
    #[prop(optional, into)]
    form: Signal<String>,
    /// Run on every click, before the box toggles, with the click itself. For what the event knows
    /// and the change does not: shift-click range selection reads `shift_key()` here, and the
    /// change arrives right after.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let checked = controllable(checked, default_checked, on_checked_change);
    let field = FieldControl::new(id, disabled);
    let node_ref = AnyNodeRef::new();

    let initial = checked.get_untracked();
    on_form_reset(node_ref, move || checked.set(initial));

    let toggle = move |e: ev::MouseEvent| {
        if field.disabled.get() {
            return;
        }
        if let Some(cb) = on_click {
            cb.run(e);
        }
        let was = checked.get_untracked();
        let next = next_checked(was, indeterminate.get_untracked());
        checked.set(next);
        if was == next
            && let Some(on_change) = on_checked_change
        {
            on_change.run(next);
        }
    };

    view! {
        <button
            node_ref=node_ref
            data-slot="checkbox"
            type="button"
            id=move || field.id.get()
            role="checkbox"
            aria-checked=move || {
                if indeterminate.get() {
                    "mixed"
                } else if checked.get() {
                    "true"
                } else {
                    "false"
                }
            }
            data-state=move || checkbox_state(checked.get(), indeterminate.get())
            disabled=move || field.disabled.get()
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            on:click=toggle
            class=class
        >
            {children()}
        </button>
        <FormInput
            control=node_ref
            input_type="checkbox"
            name=name
            value=value
            form=form
            checked=Signal::derive(move || checked.get() && !indeterminate.get())
            required=required
            disabled=field.disabled
        />
    }
}
