use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn ButtonRoot(
    #[prop(default = None, into)] node_ref: Option<AnyNodeRef>,
    #[prop(default = None, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_default();

    view! {
        <button
            data-slot="button"
            disabled=move || disabled.get()
            class=class
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
            node_ref=node_ref
        >
            {children()}
        </button>
    }
}
