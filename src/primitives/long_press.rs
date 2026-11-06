//! A press that is held still long enough to mean something.
//!
//! A mouse has a right button for "more"; a finger has only a longer press. This is the timer
//! behind that: started from a `pointerdown`, it fires once after a delay unless the pointer has
//! travelled further than a finger's natural tremor, or let go, or been taken away by the system
//! (`pointercancel`, which a touch scroll produces).
//!
//! Only touch and pen presses count. A held mouse button is a drag or a text selection, and
//! anything that wants a mouse gesture has `contextmenu`.
//!
//! The press can start on a child of the element carrying the handler: the point reported is
//! where the pointer went down, not where the element is. The gesture is followed through
//! [`crate::primitives::drag::use_drag`], so the listeners are taken off on release, on cancel
//! and on unmount.

use crate::primitives::drag::{Drag, DragEnd, DragPoint, use_drag};
use leptos::{ev, prelude::*};
use std::time::Duration;

/// How long a press is held before it counts, in milliseconds.
pub const DEFAULT_DELAY_MS: f64 = 450.0;
/// How far, in pixels, the pointer may wander before the press is a drag or a scroll instead.
pub const MOVE_TOLERANCE: f64 = 10.0;

/// Where the held press went down, in viewport coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LongPressPoint {
    pub client_x: f64,
    pub client_y: f64,
}

/// Whether a pointer of this `PointerEvent.pointerType` can long-press. Mouse cannot.
pub fn can_long_press(pointer_type: &str) -> bool {
    matches!(pointer_type, "touch" | "pen")
}

/// Whether a pointer that has moved `(dx, dy)` from its press has gone further than `tolerance`
/// and so abandoned the press. Exactly at the tolerance is still a hold.
pub fn has_drifted(dx: f64, dy: f64, tolerance: f64) -> bool {
    dx * dx + dy * dy > tolerance * tolerance
}

/// A long-press detector. `Copy`, so it can be handed to as many event handlers as needed.
#[derive(Copy, Clone)]
pub struct LongPress {
    /// Whether a touch or pen press is down, from the press until its release — including after
    /// the long press has fired. Mark the element with it to hold off text selection meanwhile.
    pub pressed: RwSignal<bool>,

    delay: Signal<f64>,
    on_trigger: Callback<LongPressPoint>,
    drag: Drag,
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    origin: StoredValue<LongPressPoint>,
}

/// Calls `on_trigger` once for each touch or pen press held still for the delay.
///
/// Attach it yourself: `on:pointerdown=move |e| long_press.start(&e)`. Guards such as `disabled`
/// belong with the caller, as they do for [`use_drag`].
pub fn use_long_press(on_trigger: impl Fn(LongPressPoint) + Send + Sync + 'static) -> LongPress {
    let timer: StoredValue<Option<TimeoutHandle>, LocalStorage> = StoredValue::new_local(None);
    let origin = StoredValue::new(LongPressPoint::default());
    let tolerance = MOVE_TOLERANCE;

    let cancel = move || {
        if let Some(handle) = timer.get_value() {
            handle.clear();
            timer.set_value(None);
        }
    };

    let drag = use_drag(move |point: DragPoint| {
        if has_drifted(point.dx, point.dy, tolerance) {
            cancel();
        }
    })
    .on_end(move |_: DragEnd| cancel());

    on_cleanup(cancel);

    LongPress {
        pressed: drag.active,
        delay: Signal::stored(DEFAULT_DELAY_MS),
        on_trigger: Callback::new(on_trigger),
        drag,
        timer,
        origin,
    }
}

impl LongPress {
    /// How long a press is held before it fires, in milliseconds. Read at each press.
    pub fn delay(mut self, delay: impl Into<Signal<f64>>) -> Self {
        self.delay = delay.into();
        self
    }

    /// Begins timing a press from a `pointerdown`. A mouse press is ignored.
    pub fn start(&self, event: &ev::PointerEvent) {
        if !can_long_press(&event.pointer_type()) {
            return;
        }

        self.cancel();

        let point = LongPressPoint {
            client_x: event.client_x() as f64,
            client_y: event.client_y() as f64,
        };
        self.origin.set_value(point);
        self.drag.start(event);

        let this = *self;
        let delay = Duration::from_millis(self.delay.get_untracked().max(0.0) as u64);

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                this.timer.set_value(None);
                if this.drag.active.get_untracked() {
                    this.on_trigger.run(this.origin.get_value());
                }
            },
            delay,
        ) {
            self.timer.set_value(Some(handle));
        }
    }

    /// Abandons the press without firing, if it has not fired already. The gesture's listeners
    /// stay until the pointer is released.
    pub fn cancel(&self) {
        if let Some(handle) = self.timer.get_value() {
            handle.clear();
            self.timer.set_value(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_and_pen_can_long_press_but_a_mouse_cannot() {
        assert!(can_long_press("touch"));
        assert!(can_long_press("pen"));
        assert!(!can_long_press("mouse"));
        assert!(!can_long_press(""));
    }

    #[test]
    fn a_press_within_the_tolerance_is_still_a_hold() {
        assert!(!has_drifted(0.0, 0.0, MOVE_TOLERANCE));
        assert!(!has_drifted(6.0, 8.0, MOVE_TOLERANCE));
        assert!(!has_drifted(-10.0, 0.0, MOVE_TOLERANCE));
    }

    #[test]
    fn a_press_beyond_the_tolerance_is_abandoned_in_any_direction() {
        assert!(has_drifted(11.0, 0.0, MOVE_TOLERANCE));
        assert!(has_drifted(0.0, -11.0, MOVE_TOLERANCE));
        assert!(has_drifted(8.0, 8.0, MOVE_TOLERANCE));
    }
}
