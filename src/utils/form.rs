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
    on_form_reset_in(node, Signal::derive(String::new), handler);
}

/// [`on_form_reset`] for a control that may belong to a form it is not inside: a non-empty `form`
/// names the `<form>` by id and is listened to instead of the ancestor. Resolved when the node
/// mounts and again whenever `form` changes; a form that is not in the document yet at that
/// moment is not found.
pub fn on_form_reset_in(
    node: AnyNodeRef,
    form: Signal<String>,
    handler: impl Fn() + Send + Sync + 'static,
) {
    let listener = StoredValue::new_local(None::<(Element, Closure<dyn FnMut()>)>);
    let handler = std::sync::Arc::new(handler);

    let detach = move || {
        if let Some(Some((form, closure))) = listener.try_update_value(|held| held.take()) {
            let _ =
                form.remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());
        }
    };

    Effect::new(move |_| {
        let id = form.get();
        let Some(node) = node.get() else {
            return;
        };
        let target = if id.is_empty() {
            closest_form(&node)
        } else {
            document().get_element_by_id(&id)
        };

        let held = listener.with_value(|held| held.as_ref().map(|(form, _)| form.clone()));
        if held == target {
            return;
        }
        detach();
        let Some(target) = target else {
            return;
        };

        let handler = handler.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let handler = handler.clone();
            set_timeout(move || handler(), Duration::ZERO);
        });
        let _ = target.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());
        listener.set_value(Some((target, closure)));
    });

    on_cleanup(detach);
}

/// Puts a native text control back to its default when its form is reset, and tells the `model`
/// bound to it.
///
/// The default is read *at the moment of the reset*, not captured at mount, so a value that
/// arrives late — a profile loaded after the form rendered — is what the form returns to, as with
/// React's `defaultValue`. Callers pass `default_value` when they set one and otherwise the
/// current `value` prop (empty when neither — an empty `default_value` is unset): never the `model`'s own earlier contents, which is
/// what the reader has been editing. The browser resets the control itself to its HTML default,
/// which a control whose value is written as a property never had, so the result is written to
/// both the element and `model`. `form` is the control's `form` prop; see [`on_form_reset_in`].
pub fn on_native_reset(
    node: AnyNodeRef,
    form: Signal<String>,
    model: Option<RwSignal<String>>,
    default_value: Signal<String>,
    value: Signal<String>,
) {
    on_form_reset_in(node, form, move || {
        let default = Some(default_value.get_untracked())
            .filter(|default| !default.is_empty())
            .unwrap_or_else(|| value.get_untracked());
        if let Some(model) = model {
            model.set(default.clone());
        }
        if let Some(node) = node.get_untracked() {
            let _ = js_sys::Reflect::set(&node, &"value".into(), &default.into());
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
