//! Whether the last thing the user did was press a key or point at something.
//!
//! A layer that closes should hand focus back to what opened it only when the keyboard closed it.
//! After Escape, or Enter on an item, focus has nowhere else to be and losing it strands the next
//! Tab at the top of the page. After a mouse or touch close the pointer already said where the
//! user is looking: pulling focus back onto the trigger paints a focus ring there, scrolls the
//! page to it if it was off screen, and leaves a trigger focused that answers ArrowDown by
//! opening again.
//!
//! The close itself carries no such information — `is_open` goes false the same way for an
//! outside press and for Escape — so this remembers the last *input* instead, from capture-phase
//! listeners on the window. Capture, so the flag is already right when a bubble-phase handler (the
//! dismissable layer's Escape, an item's click) reacts to the same event.

use leptos::{
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use std::cell::Cell;

thread_local! {
    static KEYBOARD: Cell<bool> = const { Cell::new(false) };
    static INSTALLED: Cell<bool> = const { Cell::new(false) };
}

/// Starts listening, once. Idempotent, so every layer's root can call it without coordinating.
///
/// The listeners live for the page, like the dismissable layer's: there is one pair, and nothing
/// is gained by taking them off.
pub fn track_input_modality() {
    if INSTALLED.with(|installed| installed.replace(true)) {
        return;
    }

    let window = window();
    let listen = |name: &str, keyboard: bool| {
        let handler = Closure::<dyn FnMut()>::new(move || KEYBOARD.with(|flag| flag.set(keyboard)));
        let _ = window.add_event_listener_with_callback_and_bool(
            name,
            handler.as_ref().unchecked_ref(),
            true,
        );
        handler.forget();
    };

    listen("keydown", true);
    listen("pointerdown", false);
}

/// Whether the most recent input was a key press. False before any input at all, so a layer
/// closed from code on a page nobody has touched does not move focus.
pub fn last_input_was_keyboard() -> bool {
    KEYBOARD.with(Cell::get)
}
