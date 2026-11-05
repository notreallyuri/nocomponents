//! Taking part in a native `<form>`.
//!
//! A `<button role="checkbox">` or a `<div role="slider">` is invisible to a form: `FormData`, a
//! plain `<form method=post>`, the browser's `required` validation and `form.reset()` all walk the
//! form's *native* controls and find nothing. The fix is the one Radix uses — a native control the
//! root keeps in step with its own state, next to the one the reader sees — and this module is the
//! parts of it that every root repeats: [`FormInput`], the shadow control, [`use_in_form`], and
//! [`on_form_reset`].
//!
//! The shadow is only rendered when it can matter: the control sits inside a `<form>`, or it was
//! given a `name` or a `form`. Elsewhere it would be an input nobody reads.

use leptos::{
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::{Element, HtmlElement};

/// Invisible, inert and out of the layout, and still a real control: `display: none` and
/// `visibility: hidden` would both exempt it from validation, and then a `required` checkbox that
/// is not ticked would submit. Translated back over the node it shadows so the browser's
/// validation bubble points at the control the reader can see.
pub const SHADOW_STYLE: &str = "position: absolute; pointer-events: none; opacity: 0; margin: 0; transform: translateX(-100%);";

/// `None` for an empty string, which is how these props say "unset": an attribute written empty
/// is not the same as an attribute left off.
pub fn non_empty(value: Signal<String>) -> Option<String> {
    Some(value.get()).filter(|value| !value.is_empty())
}

/// Whether the node is inside a `<form>`, known once it has mounted and `false` until then.
pub fn use_in_form(node: AnyNodeRef) -> Signal<bool> {
    let in_form = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(node) = node.get() {
            in_form.set(closest_form(&node).is_some());
        }
    });

    in_form.into()
}

fn closest_form(node: &Element) -> Option<Element> {
    node.closest("form").ok().flatten()
}

/// Runs `handler` when the form around `node` is reset — the moment a control should go back to
/// the value it started with, which for an uncontrolled one is its `default_*`.
///
/// `reset` fires *before* the browser puts the native controls back, and a signal written inside
/// the event would have its effect run and then be overwritten by that. So the handler runs a tick
/// later, once the form has finished; the shadows carry their state as the `checked` attribute as
/// well as the property, so that a reset which the handler leaves unchanged restores them to what
/// the root already says. The listener is on the form itself and is removed with the owner.
pub fn on_form_reset(node: AnyNodeRef, handler: impl Fn() + Send + Sync + 'static) {
    let listener = StoredValue::new_local(None::<(Element, Closure<dyn FnMut()>)>);
    let handler = std::sync::Arc::new(handler);

    Effect::new(move |_| {
        let Some(form) = node.get().and_then(|node| closest_form(&node)) else {
            return;
        };
        if listener.with_value(|held| held.is_some()) {
            return;
        }

        let handler = handler.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let handler = handler.clone();
            set_timeout(move || handler(), Duration::ZERO);
        });
        let _ = form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());
        listener.set_value(Some((form, closure)));
    });

    on_cleanup(move || {
        if let Some(Some((form, closure))) = listener.try_update_value(|held| held.take()) {
            let _ =
                form.remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());
        }
    });
}

/// The native control behind a custom one: `aria-hidden`, not a tab stop, not clickable, sized
/// like the node it shadows.
///
/// `control` is that node. `input_type` is `"checkbox"` or `"radio"`; `checked`, `name`, `value`,
/// `required`, `disabled` and `form` are written straight onto it, so what the form submits is what
/// the root's state says. An empty `value` leaves the attribute off, which a checkbox or radio
/// submits as `"on"`.
#[component]
pub fn FormInput(
    control: AnyNodeRef,
    input_type: &'static str,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let in_form = use_in_form(control);
    let input_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if let (Some(input), Some(control)) = (input_ref.get(), control.get())
            && let (Ok(input), Ok(control)) = (
                input.dyn_into::<HtmlElement>(),
                control.dyn_into::<HtmlElement>(),
            )
        {
            let style = input.style();
            let _ = style.set_property("width", &format!("{}px", control.offset_width()));
            let _ = style.set_property("height", &format!("{}px", control.offset_height()));
        }
    });

    let shown = move || in_form.get() || !name.get().is_empty() || !form.get().is_empty();

    view! {
        <Show when=shown>
            <input
                node_ref=input_ref
                type=input_type
                aria-hidden="true"
                tabindex="-1"
                style=SHADOW_STYLE
                name=move || non_empty(name)
                value=move || non_empty(value)
                form=move || non_empty(form)
                required=move || required.get()
                disabled=move || disabled.get()
                checked=move || checked.get()
                prop:checked=move || checked.get()
            />
        </Show>
    }
}
