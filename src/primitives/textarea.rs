use crate::{cn, primitives::field::FieldControl, utils::form::non_empty};
use leptos::{prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

#[component]
pub fn TextareaRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = None, into)] model: Option<RwSignal<String>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = false)] auto_resize: bool,
    /// The name the text is submitted under. Empty leaves it out of a form's data.
    #[prop(optional, into)]
    name: Signal<String>,
    /// The browser refuses to submit the form while this is empty.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Focusable and selectable but not editable, and still submitted.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    #[prop(optional, into)] placeholder: Signal<String>,
    /// A browser autofill hint such as `"street-address"`. Empty leaves the attribute off.
    #[prop(optional, into)]
    autocomplete: Signal<String>,
    /// The visible height in lines. Unset leaves the browser's default.
    #[prop(default = None, into)]
    rows: Option<u32>,
    /// The `id` of a `<form>` the textarea belongs to when it is not inside one. Empty means the
    /// one around it.
    #[prop(optional, into)]
    form: Signal<String>,
    /// The `<textarea>` itself, for a caller that has to reach the element — the selection is the
    /// one piece of a textarea's state that lives nowhere but the DOM, so a toolbar that formats
    /// what the reader has highlighted cannot be written without it.
    #[prop(optional)]
    node_ref: Option<AnyNodeRef>,
    /// Every edit, with the text as it now stands — for a caller that reacts to typing without
    /// owning the value through `model`.
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,
) -> impl IntoView {
    let field = FieldControl::new(id, disabled);
    // One ref whether or not the caller wanted it: auto-resize measures through the same node.
    let node_ref = node_ref.unwrap_or_default();

    let resize = move || {
        if auto_resize
            && let Some(node) = node_ref.get()
            && let Ok(el) = node.dyn_into::<HtmlElement>()
        {
            let border_offset = el.offset_height() - el.client_height();
            let _ = el.style().set_property("overflow-y", "hidden");
            let _ = el.style().set_property("height", "auto");
            let target_height = el.scroll_height() + border_offset;
            let _ = el
                .style()
                .set_property("height", &format!("{}px", target_height));
            let _ = el.style().remove_property("overflow-y");
        }
    };

    Effect::new(move |_| {
        if auto_resize {
            if let Some(m) = model {
                m.track();
            } else {
                value.track();
            }

            set_timeout(resize, std::time::Duration::from_millis(10));
        }
    });

    view! {
        <textarea
            data-slot="textarea"
            node_ref=node_ref
            id=move || field.id.get()
            disabled=move || field.disabled.get()
            name=move || non_empty(name)
            form=move || non_empty(form)
            required=move || required.get()
            readonly=move || readonly.get()
            placeholder=move || non_empty(placeholder)
            autocomplete=move || non_empty(autocomplete)
            rows=rows
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            prop:value=move || if let Some(m) = model { m.get() } else { value.get() }
            on:input=move |ev| {
                let text = event_target_value(&ev);
                if let Some(m) = model {
                    m.set(text.clone());
                }
                if let Some(cb) = on_input {
                    cb.run(text);
                }
                if auto_resize {
                    resize();
                }
            }
            class=move || {
                cn!(
                    if auto_resize { "field-sizing-content resize-none" } else { "" },
                    class.get()
                )
            }
        />
    }
}
