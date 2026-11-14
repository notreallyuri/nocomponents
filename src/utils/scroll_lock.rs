//! Freezes the page behind a modal layer, for as long as any layer asks.
//!
//! `overflow: hidden` on the root and the body stops the wheel and the keyboard, and Chrome's
//! touch scrolling with them. iOS Safari ignores it for touch: a finger dragged over a scrim or a
//! sheet's header still moves the page behind, and so does a finger that reaches the end of a
//! scrolling list and carries on. So while locked a non-passive `touchmove` listener on the
//! document cancels every vertical move unless it starts in something that can still scroll that
//! way; `overscroll-behavior: contain` on that scroller keeps the rest of the gesture inside it.
//!
//! Touches that start in an `input`, `textarea`, `select` or `[contenteditable]` are never
//! cancelled, so a caret or a selection handle can be dragged in a field that does not scroll.
//!
//! The lock is counted. A sheet opened over a dialog must not unfreeze the page when it closes, so
//! the first layer to ask applies it and the last to let go removes it. The inline `overflow`,
//! `overflow-x` and `overflow-y` of `<html>` and `<body>`, priorities included, are saved then and
//! put back exactly.

use leptos::{
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use std::cell::{Cell, RefCell};
use web_sys::{AddEventListenerOptions, Element, HtmlElement, TouchEvent};

/// How many layers hold the lock. `acquire` says whether it was the first, `release` whether it
/// was the last; a release with nothing held is ignored.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LockCount(usize);

impl LockCount {
    pub fn acquire(&mut self) -> bool {
        self.0 += 1;
        self.0 == 1
    }

    pub fn release(&mut self) -> bool {
        if self.0 == 0 {
            return false;
        }
        self.0 -= 1;
        self.0 == 0
    }

    pub fn held(&self) -> usize {
        self.0
    }
}

/// The scroll position of the nearest ancestor that scrolls vertically.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scroller {
    pub top: f64,
    pub client: f64,
    pub height: f64,
}

/// Whether a touch that began in an editable element is left alone; see [`touch_blocked`].
pub fn touch_blocked(editable: bool, scroller: Option<Scroller>, dy: f64) -> bool {
    !editable && blocks_touch(scroller, dy)
}

/// Whether a vertical touch move of `dy` pixels (positive is the finger going down, which scrolls
/// the content up) must be cancelled: nothing under the finger scrolls, or it is already at the
/// end the finger is pushing towards.
pub fn blocks_touch(scroller: Option<Scroller>, dy: f64) -> bool {
    match scroller {
        None => true,
        Some(_) if dy == 0.0 => false,
        Some(scroller) if dy > 0.0 => scroller.top <= 0.0,
        Some(scroller) => scroller.top + scroller.client >= scroller.height - 1.0,
    }
}

const OVERFLOW: [&str; 3] = ["overflow", "overflow-x", "overflow-y"];

/// What a touch must never be cancelled on: a field being typed in, whose caret and selection
/// handles are dragged with the same gesture the page would scroll by.
const EDITABLE: &str = "input, textarea, select, [contenteditable]";

type Inline = Vec<(&'static str, String, String)>;

struct Saved {
    root: Inline,
    body: Inline,
    touchstart: Closure<dyn FnMut(TouchEvent)>,
    touchmove: Closure<dyn FnMut(TouchEvent)>,
}

thread_local! {
    static COUNT: Cell<LockCount> = const { Cell::new(LockCount(0)) };
    static SAVED: RefCell<Option<Saved>> = const { RefCell::new(None) };
    static START: Cell<(f64, f64)> = const { Cell::new((0.0, 0.0)) };
}

/// The attribute on `<html>` while the page is frozen, so a stylesheet or a test can see it.
pub const LOCK_ATTR: &str = "data-scroll-locked";

fn scroller_of(target: &Element) -> Option<Scroller> {
    let body = document().body();
    let mut current = Some(target.clone());
    while let Some(el) = current {
        if body.as_deref().is_some_and(|body| body == &el) {
            return None;
        }
        if el.scroll_height() > el.client_height() + 1
            && let Ok(Some(style)) = window().get_computed_style(&el)
            && matches!(
                style.get_property_value("overflow-y").as_deref(),
                Ok("auto" | "scroll" | "overlay")
            )
        {
            return Some(Scroller {
                top: el.scroll_top() as f64,
                client: el.client_height() as f64,
                height: el.scroll_height() as f64,
            });
        }
        current = el.parent_element();
    }
    None
}

fn lock() {
    let doc = document();
    let (Some(root), Some(body)) = (doc.document_element(), doc.body()) else {
        return;
    };
    let Ok(root) = root.dyn_into::<HtmlElement>() else {
        return;
    };
    let saved_root = save(&root);
    let saved_body = save(&body);
    let _ = root
        .style()
        .set_property_with_priority("overflow", "hidden", "important");
    let _ = body
        .style()
        .set_property_with_priority("overflow", "hidden", "important");
    let _ = root.set_attribute(LOCK_ATTR, "");

    let touchstart = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
        if let Some(touch) = e.touches().get(0) {
            START.with(|start| start.set((touch.client_x() as f64, touch.client_y() as f64)));
        }
    });
    let touchmove = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
        if e.touches().length() != 1 || !e.cancelable() {
            return;
        }
        let Some(touch) = e.touches().get(0) else {
            return;
        };
        let (x, y) = START.with(Cell::get);
        let dx = touch.client_x() as f64 - x;
        let dy = touch.client_y() as f64 - y;
        if dx.abs() > dy.abs() {
            return;
        }
        let target = e
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok());
        let editable = target
            .as_ref()
            .is_some_and(|target| matches!(target.closest(EDITABLE), Ok(Some(_))));
        let scroller = target.as_ref().and_then(scroller_of);
        if touch_blocked(editable, scroller, dy) {
            e.prevent_default();
        }
    });

    let passive = AddEventListenerOptions::new();
    passive.set_passive(true);
    let active = AddEventListenerOptions::new();
    active.set_passive(false);
    let _ = doc.add_event_listener_with_callback_and_add_event_listener_options(
        "touchstart",
        touchstart.as_ref().unchecked_ref(),
        &passive,
    );
    let _ = doc.add_event_listener_with_callback_and_add_event_listener_options(
        "touchmove",
        touchmove.as_ref().unchecked_ref(),
        &active,
    );

    SAVED.with(|saved| {
        *saved.borrow_mut() = Some(Saved {
            root: saved_root,
            body: saved_body,
            touchstart,
            touchmove,
        });
    });
}

fn unlock() {
    let Some(saved) = SAVED.with(|saved| saved.borrow_mut().take()) else {
        return;
    };
    let doc = document();
    let _ = doc.remove_event_listener_with_callback(
        "touchstart",
        saved.touchstart.as_ref().unchecked_ref(),
    );
    let _ = doc
        .remove_event_listener_with_callback("touchmove", saved.touchmove.as_ref().unchecked_ref());
    if let Some(root) = doc.document_element()
        && let Ok(root) = root.dyn_into::<HtmlElement>()
    {
        restore(&root, &saved.root);
        let _ = root.remove_attribute(LOCK_ATTR);
    }
    if let Some(body) = doc.body() {
        restore(&body, &saved.body);
    }
}

fn save(el: &HtmlElement) -> Inline {
    let style = el.style();
    OVERFLOW
        .iter()
        .map(|name| {
            (
                *name,
                style.get_property_value(name).unwrap_or_default(),
                style.get_property_priority(name),
            )
        })
        .collect()
}

fn restore(el: &HtmlElement, saved: &Inline) {
    let style = el.style();
    for name in OVERFLOW {
        let _ = style.remove_property(name);
    }
    for (name, value, priority) in saved {
        if !value.is_empty() {
            let _ = style.set_property_with_priority(name, value, priority);
        }
    }
}

/// Holds the page still while `locked` is true. Call it once per layer, inside the owner that
/// layer renders under; the lock is let go when the signal turns false and when the owner goes.
pub fn use_scroll_lock(locked: Signal<bool>) {
    let held = StoredValue::new(false);

    Effect::new(move |_| {
        let want = locked.get();
        if want == held.get_value() {
            return;
        }
        held.set_value(want);
        let mut count = COUNT.with(Cell::get);
        let edge = if want {
            count.acquire()
        } else {
            count.release()
        };
        COUNT.with(|cell| cell.set(count));
        if edge {
            if want {
                lock();
            } else {
                unlock();
            }
        }
    });

    on_cleanup(move || {
        if held.try_get_value().unwrap_or(false) {
            let mut count = COUNT.with(Cell::get);
            if count.release() {
                unlock();
            }
            COUNT.with(|cell| cell.set(count));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_first_and_last_layer_change_the_page() {
        let mut count = LockCount::default();
        assert!(count.acquire());
        assert!(!count.acquire());
        assert!(!count.release());
        assert!(count.release());
        assert!(!count.release());
        assert_eq!(count.held(), 0);
    }

    #[test]
    fn a_move_with_nothing_to_scroll_is_cancelled() {
        assert!(blocks_touch(None, 10.0));
        assert!(blocks_touch(None, -10.0));
    }

    #[test]
    fn a_touch_in_an_editable_element_is_never_cancelled() {
        assert!(!touch_blocked(true, None, 10.0));
        assert!(touch_blocked(false, None, 10.0));
    }

    #[test]
    fn a_scroller_keeps_the_moves_it_can_still_take() {
        let middle = Scroller {
            top: 50.0,
            client: 100.0,
            height: 400.0,
        };
        assert!(!blocks_touch(Some(middle), 10.0));
        assert!(!blocks_touch(Some(middle), -10.0));
    }

    #[test]
    fn a_scroller_at_an_end_does_not_hand_the_move_to_the_page() {
        let top = Scroller {
            top: 0.0,
            client: 100.0,
            height: 400.0,
        };
        assert!(blocks_touch(Some(top), 10.0));
        assert!(!blocks_touch(Some(top), -10.0));
        let bottom = Scroller {
            top: 300.0,
            client: 100.0,
            height: 400.0,
        };
        assert!(blocks_touch(Some(bottom), -10.0));
        assert!(!blocks_touch(Some(bottom), 10.0));
    }
}
