//! A button, and the second shape a button comes in.
//!
//! There is no state here and no context to publish. What makes this a primitive rather than a
//! bare `<button>` is the `disabled` it resolves — its own, or the field's around it — and that it
//! owns both shapes: `render` turns the button into whatever the caller returned, an `<a>` almost
//! always, since a link has to stay a link for middle-click and copy-address.
//!
//! An `<a>` has no `disabled` attribute to wear, so for that shape the state is written onto the
//! node as `aria-disabled` and `data-disabled` instead. That is the trick
//! [`crate::primitives::floating`] uses for its trigger's ARIA and for the same reason: the
//! element belongs to the caller, and the node ref is all the shapes have in common.
//!
//! `type` is a prop rather than an attribute left to the caller because HTML's default is
//! `submit` — a button that means nothing in particular submits the form it happens to be inside.
//! Here the default is [`ButtonType::Button`] and a submit is asked for by name.

use crate::{primitives::field::field_disabled, utils::types::StyledRender};
use leptos::{either::Either, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// What the button does to a form around it.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonType {
    /// Nothing on its own — the default here, though not HTML's.
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

#[component]
pub fn ButtonRoot(
    /// Pass one in when something outside needs this element too — a floating layer measuring the
    /// trigger it hangs off, say.
    #[prop(default = None, into)]
    node_ref: Option<AnyNodeRef>,
    #[prop(default = None, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: Signal<String>,
    /// Resolved against any [`crate::primitives::field`] around it, so a button inside a disabled
    /// field is disabled without being told twice.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional)] r#type: ButtonType,
    /// Render as something else. Handed the class this would have rendered, the node ref the
    /// state attributes go on, and the resolved `disabled` — a link cannot wear the attribute, so
    /// what it does about it is the caller's to decide.
    #[prop(default = None, into)]
    render: Option<Callback<StyledRender, AnyView>>,
    #[prop(default = None)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_default();
    let disabled = field_disabled(disabled);

    // The element is the caller's, so the attributes go onto the node rather than being declared
    // here — the same shape `sidebar` and `floating` use for their own.
    if render.is_some() {
        Effect::new(move |_| {
            let Some(el) = node_ref.get() else {
                return;
            };

            let _ = el.set_attribute("data-slot", "button");

            if disabled.get() {
                let _ = el.set_attribute("aria-disabled", "true");
                let _ = el.set_attribute("data-disabled", "true");
            } else {
                let _ = el.remove_attribute("aria-disabled");
                let _ = el.remove_attribute("data-disabled");
            }
        });
    }

    // Refused here rather than left to the styled layer's `disabled:pointer-events-none`: the
    // classes are the caller's to replace, and a button someone else styled has to refuse it too.
    let on_click = move |ev: ev::MouseEvent| {
        if disabled.get() {
            ev.prevent_default();
            return;
        }

        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    match render {
        Some(render_fn) => Either::Left(render_fn.run(StyledRender {
            class,
            node_ref,
            disabled,
        })),
        None => Either::Right(view! {
            <button
                data-slot="button"
                type=r#type.as_str()
                node_ref=node_ref
                disabled=move || disabled.get()
                data-disabled=move || disabled.get().then_some("true")
                on:click=on_click
                class=class
            >
                {children.map(|children| children())}
            </button>
        }),
    }
}
