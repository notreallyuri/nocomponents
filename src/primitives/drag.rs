//! One pointer gesture, tracked on the window.
//!
//! Four primitives had each written this out: press on something, follow the pointer wherever it
//! goes — including outside the element, which is why the listeners go on the window and not on
//! the target — and stop on release. What differs between them is arithmetic; what does not is the
//! bookkeeping, and the bookkeeping is where the bugs are. Two listeners per gesture, taken off on
//! release *and* on unmount, never left behind by a second press, and a running sample of where
//! the pointer was a moment ago so a flick can be told from a haul.
//!
//! A gesture also ends on `pointercancel` — what a touch drag gets when the system takes the
//! pointer away, mid-scroll or on an incoming call. Every hand-rolled copy of this missed it and
//! left its listeners installed.

use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::Element;

/// Where the pointer is, in each of the forms a caller has wanted.
#[derive(Clone, Copy, Debug, Default)]
pub struct DragPoint {
    pub client_x: f64,
    pub client_y: f64,
    /// Movement since the press. What a gesture that moves something *by* an amount wants.
    pub dx: f64,
    pub dy: f64,
    /// Where the pointer sits inside the reference box: 0 at its left or top edge, 1 at its right
    /// or bottom. Deliberately unclamped, so a caller can tell an overshoot from an edge — and
    /// measured live, so a page that scrolls mid-drag does not shift the frame under the gesture.
    pub fraction_x: f64,
    pub fraction_y: f64,
    /// The modifiers held for *this* move, not for the press that began the gesture — every
    /// pointer event carries its own, so taking or releasing a key partway through changes what
    /// the rest of the drag means without the caller listening for keys itself.
    ///
    /// Only the two a caller has wanted so far. `ctrl` and `meta` go here when something needs
    /// them; there is nothing to work out, only a line to add.
    pub shift: bool,
    pub alt: bool,
}

/// The release, and how the pointer was travelling when it came.
#[derive(Clone, Copy, Debug, Default)]
pub struct DragEnd {
    pub point: DragPoint,
    /// Pixels per millisecond over the last movement, signed. Zero if the pointer never moved.
    pub velocity_x: f64,
    pub velocity_y: f64,
    /// Milliseconds since the pointer last moved. A pause before release is a placement rather
    /// than a throw, which is the difference between dropping a panel and flicking it away.
    pub idle: f64,
}

/// What a gesture carries between its press and its release. Not `Send`, because the pressed
/// element is not, which is why it lives in a local `StoredValue`.
#[derive(Default)]
struct Gesture {
    listeners: Vec<WindowListenerHandle>,
    origin: (f64, f64),
    /// Where the pointer was at the previous move, and when.
    sample: (f64, f64, f64),
    velocity: (f64, f64),
    /// The element the press landed on, which is the reference box unless one was named.
    pressed: Option<Element>,
}

/// A pointer gesture. `Copy`, so it can be handed to as many event handlers as needed.
#[derive(Copy, Clone)]
pub struct Drag {
    /// Whether a gesture is in progress. Mark the element with it — `data-dragging` — rather than
    /// keeping a second flag beside it.
    pub active: RwSignal<bool>,

    reference: Option<AnyNodeRef>,
    on_move: Callback<DragPoint>,
    on_start: Option<Callback<DragPoint>>,
    on_end: Option<Callback<DragEnd>>,

    gesture: StoredValue<Gesture, LocalStorage>,
}

/// Tracks a pointer gesture, calling `on_move` for every move until it is released.
///
/// Attach it yourself: `on:pointerdown=move |e| drag.start(&e)`. Each caller guards its own press
/// — a slider ignores one while disabled, a drawer ignores one that landed in a scroller — and a
/// guard belongs with the thing it is guarding, not in here.
pub fn use_drag(on_move: impl Fn(DragPoint) + Send + Sync + 'static) -> Drag {
    let drag = Drag {
        active: RwSignal::new(false),
        reference: None,
        on_move: Callback::new(on_move),
        on_start: None,
        on_end: None,
        gesture: StoredValue::new_local(Gesture::default()),
    };

    // The handles live in the gesture, which every copy of this shares, so cleaning up through
    // the value captured here reaches whatever the builders returned.
    on_cleanup(move || drag.stop());

    drag
}

impl Drag {
    /// The box `fraction_x` and `fraction_y` are measured against. Defaults to the element the
    /// press landed on, which is right when that element *is* the surface being dragged over and
    /// wrong when the press is on a wrapper — a slider's press lands on its root, but its
    /// geometry belongs to the track.
    pub fn against(mut self, reference: AnyNodeRef) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Run at the press, before any movement, with `dx` and `dy` at zero. Where a click and a
    /// drag become one gesture: a slider's thumb jumps to the press and is then dragged from it.
    pub fn on_start(mut self, on_start: impl Fn(DragPoint) + Send + Sync + 'static) -> Self {
        self.on_start = Some(Callback::new(on_start));
        self
    }

    /// Run at the release. Also run for a cancelled gesture, so a caller that has moved something
    /// always gets the chance to settle it.
    pub fn on_end(mut self, on_end: impl Fn(DragEnd) + Send + Sync + 'static) -> Self {
        self.on_end = Some(Callback::new(on_end));
        self
    }

    fn reference_rect(&self) -> Option<web_sys::DomRect> {
        match self.reference {
            Some(reference) => reference
                .get_untracked()
                .map(|element| element.get_bounding_client_rect()),
            None => self
                .gesture
                .with_value(|gesture| gesture.pressed.clone())
                .map(|element| element.get_bounding_client_rect()),
        }
    }

    fn point(&self, event: &ev::PointerEvent) -> DragPoint {
        let (client_x, client_y) = (event.client_x() as f64, event.client_y() as f64);
        let (origin_x, origin_y) = self.gesture.with_value(|gesture| gesture.origin);

        let (fraction_x, fraction_y) = match self.reference_rect() {
            Some(rect) if rect.width() > 0.0 && rect.height() > 0.0 => (
                (client_x - rect.left()) / rect.width(),
                (client_y - rect.top()) / rect.height(),
            ),
            _ => (0.0, 0.0),
        };

        DragPoint {
            client_x,
            client_y,
            dx: client_x - origin_x,
            dy: client_y - origin_y,
            fraction_x,
            fraction_y,
            shift: event.shift_key(),
            alt: event.alt_key(),
        }
    }

    /// Begins a gesture from a `pointerdown`.
    pub fn start(&self, event: &ev::PointerEvent) {
        // A press while another gesture is somehow still live takes over rather than stacking a
        // second set of listeners on top of the first.
        self.stop();

        let client_x = event.client_x() as f64;
        let client_y = event.client_y() as f64;
        let pressed = event
            .current_target()
            .and_then(|target| target.dyn_into::<Element>().ok());

        self.gesture.update_value(|gesture| {
            gesture.origin = (client_x, client_y);
            gesture.sample = (client_x, client_y, now());
            gesture.velocity = (0.0, 0.0);
            gesture.pressed = pressed;
        });

        self.active.set(true);

        if let Some(on_start) = self.on_start {
            on_start.run(self.point(event));
        }

        let drag = *self;
        let listeners = vec![
            window_event_listener(ev::pointermove, move |event| drag.moved(&event)),
            window_event_listener(ev::pointerup, move |event| drag.finish(&event)),
            window_event_listener(ev::pointercancel, move |event| drag.finish(&event)),
        ];

        self.gesture
            .update_value(|gesture| gesture.listeners = listeners);
    }

    fn moved(&self, event: &ev::PointerEvent) {
        let client_x = event.client_x() as f64;
        let client_y = event.client_y() as f64;

        self.gesture.update_value(|gesture| {
            let (previous_x, previous_y, previous_time) = gesture.sample;
            let time = now();
            let elapsed = time - previous_time;

            // Two moves inside the same millisecond would divide by zero, and the sample they
            // would produce is noise anyway.
            if elapsed > 0.0 {
                gesture.velocity = (
                    (client_x - previous_x) / elapsed,
                    (client_y - previous_y) / elapsed,
                );
            }

            gesture.sample = (client_x, client_y, time);
        });

        self.on_move.run(self.point(event));
    }

    fn finish(&self, event: &ev::PointerEvent) {
        let point = self.point(event);
        let (velocity_x, velocity_y, idle) = self.gesture.with_value(|gesture| {
            let (velocity_x, velocity_y) = gesture.velocity;
            (velocity_x, velocity_y, now() - gesture.sample.2)
        });

        // Torn down before the callback: a caller that closes something in `on_end` must not be
        // able to leave the listeners behind.
        self.stop();

        if let Some(on_end) = self.on_end {
            on_end.run(DragEnd {
                point,
                velocity_x,
                velocity_y,
                idle,
            });
        }
    }

    /// Ends the gesture without running `on_end`. Called on release and on unmount; call it
    /// yourself to abandon a gesture partway.
    pub fn stop(&self) {
        self.gesture.update_value(|gesture| {
            for listener in gesture.listeners.drain(..) {
                listener.remove();
            }
            gesture.pressed = None;
        });

        if self.active.get_untracked() {
            self.active.set(false);
        }
    }
}

fn now() -> f64 {
    js_sys::Date::now()
}
