use crate::{cn, primitives::field::FieldControl};
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
    /// The `<textarea>` itself, for a caller that has to reach the element — the selection is the
    /// one piece of a textarea's state that lives nowhere but the DOM, so a toolbar that formats
    /// what the reader has highlighted cannot be written without it.
    #[prop(optional)]
    node_ref: Option<AnyNodeRef>,
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
            aria-describedby=move || field.described_by.get()
            aria-invalid=move || field.invalid.get().then_some("true")
            prop:value=move || if let Some(m) = model { m.get() } else { value.get() }
            on:input=move |ev| {
                if let Some(m) = model {
                    m.set(event_target_value(&ev));
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
