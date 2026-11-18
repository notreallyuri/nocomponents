//! A click listener for an element somebody else rendered.
//!
//! A part that takes `render` does not own its element, so a handler cannot ride in as an
//! `on:click` in its markup; it is attached to the node once there is one. Captured, so a part
//! that refuses the click gets to do so before any handler the caller put on the same node.

use leptos::{prelude::*, wasm_bindgen::JsCast, wasm_bindgen::closure::Closure};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, MouseEvent};

/// Runs `handler` on every click of the node `node_ref` points at, for as long as the calling
/// owner lives. The handler is `FnMut` so it may call `prevent_default` and
/// `stop_immediate_propagation` on the event it is given.
pub fn listen_click(node_ref: AnyNodeRef, handler: impl FnMut(MouseEvent) + 'static) {
    let held = StoredValue::new_local(None::<(Element, Closure<dyn FnMut(MouseEvent)>)>);
    let handler = StoredValue::new_local(Some(handler));

    Effect::new(move |_| {
        let Some(el) = node_ref.get() else {
            return;
        };
        if held.with_value(|held| held.is_some()) {
            return;
        }
        let Some(mut handler) = handler.try_update_value(|h| h.take()).flatten() else {
            return;
        };

        let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |ev: MouseEvent| handler(ev));
        let _ = el.add_event_listener_with_callback_and_bool(
            "click",
            closure.as_ref().unchecked_ref(),
            true,
        );
        held.set_value(Some((el, closure)));
    });

    on_cleanup(move || {
        if let Some(Some((el, closure))) = held.try_update_value(|held| held.take()) {
            let _ = el.remove_event_listener_with_callback_and_bool(
                "click",
                closure.as_ref().unchecked_ref(),
                true,
            );
        }
    });
}
