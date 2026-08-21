//! A sheet you can throw away with your thumb.
//!
//! Everything a drawer shares with a dialog is the dialog's: modality, the focus trap, the layer
//! stack, Escape, and the exit animation that `is_mounted` keeps alive. What this adds is the one
//! gesture a sheet does not have — drag the panel toward its own edge and let go, and it leaves.
//!
//! Two rules make that gesture feel right rather than merely work. It only travels *outward*: a
//! bottom drawer dragged upward does not lift off its edge, because there is nothing up there to
//! reveal. And a flick counts as much as a haul — releasing at speed dismisses even a short drag,
//! since throwing something away is not the same gesture as placing it, and requiring a fixed
//! distance would make a fast flick feel ignored.
//!
//! The drag is refused when the press lands inside something that is itself scrolled away from the
//! top, so a drawer containing a long list scrolls that list instead of leaving.

use crate::{
    primitives::dialog::{DialogContentRoot, DialogRoot, use_dialog},
    utils::types::Side,
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::Element;

/// How far the panel has to travel, as a fraction of its own length, before letting go dismisses.
const DISTANCE_THRESHOLD: f64 = 0.25;
/// Pixels per millisecond past which a release counts as a flick regardless of distance.
const VELOCITY_THRESHOLD: f64 = 0.4;
/// How much a panel gives up while a nested drawer sits over it, and how far it slides back from
/// its own edge. Small on purpose: the point is to show that something is behind the child, not to
/// animate the parent away.
const RECEDE_SCALE: f64 = 0.96;
const RECEDE_SHIFT: f64 = 14.0;

#[derive(Copy, Clone)]
pub struct DrawerContext {
    pub side: Side,
    /// How far the panel has been dragged from its resting place, in pixels. Never negative — see
    /// the module note about travelling outward only.
    pub offset: RwSignal<f64>,
    pub dragging: RwSignal<bool>,
    /// How many drawers are currently open inside this one. A count rather than a flag because
    /// two children can overlap while one is still animating out, and a flag would flicker.
    pub nested_open: RwSignal<usize>,
    /// The same counter belonging to the drawer this one sits inside, so a child can announce
    /// itself. `None` at the top of the stack. Holding the parent's *signal* rather than its
    /// context is what keeps `DrawerContext` a fixed size.
    parent_nested: Option<RwSignal<usize>>,
}

impl DrawerContext {
    /// Whether a nested drawer is currently sitting over this one.
    pub fn is_receded(&self) -> bool {
        self.nested_open.get() > 0
    }

    /// The panel's transform: its drag offset, or its recede when a nested drawer is over it.
    ///
    /// A drag in progress wins over the recede. The panel under the finger has to follow the
    /// finger, and there is no sensible way to be halfway between "shoved aside" and "held".
    pub fn transform(&self) -> String {
        let offset = self.offset.get();
        let receded = self.is_receded();

        // Receding moves *inward*, the opposite way from a dismissal: sliding back from its own
        // edge is what leaves the parent's far edge visible behind the child.
        let travel = match (offset != 0.0, receded) {
            (true, _) => offset,
            (false, true) => -RECEDE_SHIFT,
            (false, false) => return String::new(),
        };

        let translate = match self.side {
            Side::Bottom => format!("translate3d(0, {travel}px, 0)"),
            Side::Top => format!("translate3d(0, {}px, 0)", -travel),
            Side::Right => format!("translate3d({travel}px, 0, 0)"),
            Side::Left => format!("translate3d({}px, 0, 0)", -travel),
        };

        if receded && offset == 0.0 {
            format!("{translate} scale({RECEDE_SCALE})")
        } else {
            translate
        }
    }

    /// Which corner the recede shrinks toward: the edge facing the viewport, never the anchored
    /// one. Scaling about the centre pulls the panel's inner edge *inward*, which cancels out the
    /// shift and leaves nothing peeking past the child; scaling about the inner edge moves that
    /// edge outward and puts the slack behind the child, where it cannot be seen.
    pub fn transform_origin(&self) -> &'static str {
        match self.side {
            Side::Bottom => "center top",
            Side::Top => "center bottom",
            Side::Right => "left center",
            Side::Left => "right center",
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(self.side, Side::Top | Side::Bottom)
    }

    /// Movement along the drag axis, positive when it points away from the anchored edge.
    fn travel(&self, dx: f64, dy: f64) -> f64 {
        match self.side {
            Side::Bottom => dy,
            Side::Top => -dy,
            Side::Right => dx,
            Side::Left => -dx,
        }
    }
}

pub fn use_drawer() -> DrawerContext {
    expect_context::<DrawerContext>()
}

#[component]
pub fn DrawerRoot(
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    /// Non-modal leaves the page usable underneath: no overlay, no focus trap, and a pointer
    /// landing outside closes the drawer instead of being swallowed.
    #[prop(default = true)]
    modal: bool,
    children: Children,
) -> impl IntoView {
    // Read before providing: this resolves to the drawer this one is nested inside, if any.
    let parent_nested = use_context::<DrawerContext>().map(|parent| parent.nested_open);

    let context = DrawerContext {
        side,
        offset: RwSignal::new(0.0),
        dragging: RwSignal::new(false),
        nested_open: RwSignal::new(0),
        parent_nested,
    };

    view! {
        <DialogRoot open=open modal=modal>
            <Provider value=context>{children()}</Provider>
        </DialogRoot>
    }
}

/// Anything that owns the press already. Dragging a panel by its own buttons is not a gesture
/// anybody makes on purpose, and starting one means a press that drifts a few pixels drags the
/// whole drawer while the user is only trying to click.
const INTERACTIVE: &str = "button, a, input, textarea, select, [role='button'], [contenteditable]";

/// Whether the press landed inside something already scrolled away from its start, in which case
/// the gesture belongs to that scroller and not to the drawer.
fn press_is_inside_scrolled_content(target: &Element, panel: &Element, vertical: bool) -> bool {
    let mut node = Some(target.clone());

    while let Some(current) = node {
        if current.is_same_node(Some(panel)) {
            return false;
        }

        let scrolled = if vertical {
            current.scroll_top() > 0
        } else {
            current.scroll_left() > 0
        };

        if scrolled {
            return true;
        }

        node = current.parent_element();
    }

    false
}

#[component]
pub fn DrawerContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_drawer();
    let dialog = use_dialog();
    let panel_ref = AnyNodeRef::new();

    let move_handle = StoredValue::new_local(None::<WindowListenerHandle>);
    let up_handle = StoredValue::new_local(None::<WindowListenerHandle>);

    let stop_listening = move || {
        move_handle.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.remove();
            }
        });
        up_handle.update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.remove();
            }
        });
    };

    // A drawer that was dragged part-way and then closed some other way — Escape, the overlay —
    // must not reopen still shoved halfway off its edge.
    Effect::new(move |_| {
        if dialog.is_open.get() {
            ctx.offset.set(0.0);
        }
    });

    // Tell the drawer above us that we are here, so it can give up some size and slide back. The
    // count is nudged on transitions rather than assigned, since two children of one parent each
    // own a share of it.
    let counted = StoredValue::new(false);
    let release_parent = move || {
        if counted.get_value()
            && let Some(parent) = ctx.parent_nested
        {
            counted.set_value(false);
            parent.update(|open| *open = open.saturating_sub(1));
        }
    };

    Effect::new(move |_| {
        let is_open = dialog.is_open.get();
        let Some(parent) = ctx.parent_nested else {
            return;
        };

        if is_open && !counted.get_value() {
            counted.set_value(true);
            parent.update(|open| *open += 1);
        } else if !is_open {
            release_parent();
        }
    });

    // A drawer torn down while still open — its whole subtree closing at once — has to hand the
    // count back, or the parent stays shrunken with nothing over it.
    on_cleanup(release_parent);

    on_cleanup(stop_listening);

    let panel_element = move || {
        panel_ref
            .get_untracked()
            .and_then(|node| node.dyn_into::<Element>().ok())
    };

    let start_drag = move |e: ev::PointerEvent| {
        let Some(panel) = panel_element() else {
            return;
        };

        if let Some(target) = e.target()
            && let Ok(target) = target.dyn_into::<Element>()
            && (matches!(target.closest(INTERACTIVE), Ok(Some(_)))
                || press_is_inside_scrolled_content(&target, &panel, ctx.is_vertical()))
        {
            return;
        }

        let start_x = e.client_x() as f64;
        let start_y = e.client_y() as f64;

        let rect = panel.get_bounding_client_rect();
        let length = if ctx.is_vertical() {
            rect.height()
        } else {
            rect.width()
        };

        // The flick test needs the speed of the *last* movement, so each move is compared against
        // the sample before it. Measuring at release instead would always read zero: by then the
        // newest sample is the release position itself.
        let last_sample = StoredValue::new((0.0f64, js_sys::Date::now()));
        let last_velocity = StoredValue::new(0.0f64);

        ctx.dragging.set(true);
        stop_listening();

        move_handle.set_value(Some(window_event_listener(
            ev::pointermove,
            move |e: ev::PointerEvent| {
                let travel = ctx
                    .travel(e.client_x() as f64 - start_x, e.client_y() as f64 - start_y)
                    .max(0.0);

                let now = js_sys::Date::now();
                let (previous_travel, previous_time) = last_sample.get_value();
                let elapsed = now - previous_time;

                if elapsed > 0.0 {
                    last_velocity.set_value((travel - previous_travel) / elapsed);
                }

                last_sample.set_value((travel, now));
                ctx.offset.set(travel);
            },
        )));

        up_handle.set_value(Some(window_event_listener(ev::pointerup, move |_| {
            stop_listening();
            ctx.dragging.set(false);

            let travel = ctx.offset.get_untracked();
            let (_, last_time) = last_sample.get_value();

            // A gap since the last movement means the pointer came to rest before letting go,
            // which is a placement rather than a throw. And the velocity is signed on purpose: a
            // quick shove *back* toward the edge should not dismiss.
            let velocity = if js_sys::Date::now() - last_time < 100.0 {
                last_velocity.get_value()
            } else {
                0.0
            };

            let far_enough = length > 0.0 && travel / length > DISTANCE_THRESHOLD;
            let fast_enough = velocity > VELOCITY_THRESHOLD && travel > 0.0;

            if far_enough || fast_enough {
                dialog.close();
                // Carry on to fully closed rather than resetting to zero. Zeroing here would snap
                // the panel back to its resting place and only *then* slide it out, so a drag that
                // succeeded would visibly rewind first. The offset is cleared when the drawer next
                // opens, by the effect above.
                ctx.offset.set(length);
            } else {
                ctx.offset.set(0.0);
            }
        })));
    };

    view! {
        <div
            node_ref=panel_ref
            data-slot="drawer"
            data-side=ctx.side.as_str()
            data-dragging=move || ctx.dragging.get().then_some("true")
            data-nested=move || ctx.is_receded().then_some("true")
            // `data-state` is normally the dialog content's, but the panel is what animates here,
            // so it has to carry the attribute the animations key off.
            data-state=move || if dialog.is_open.get() { "open" } else { "closed" }
            style=move || {
                let transform = match ctx.transform().as_str() {
                    "" => String::new(),
                    value => {
                        format!("transform: {value}; transform-origin: {};", ctx.transform_origin())
                    }
                };
                // Transitions are off while a finger is down: the panel has to track the pointer
                // exactly, and easing it would feel like lag.
                if ctx.dragging.get() {
                    format!("{transform} transition: none;")
                } else {
                    transform
                }
            }
            on:pointerdown=start_drag
            class=class
        >
            <DialogContentRoot style="height: 100%; width: 100%; display: flex; flex-direction: column;">
                {children()}
            </DialogContentRoot>
        </div>
    }
}

/// The grab bar. Purely an affordance — the whole panel is draggable — but a drawer without one
/// does not look draggable.
#[component]
pub fn DrawerHandleRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_drawer();

    view! {
        <div
            data-slot="drawer-handle"
            data-side=ctx.side.as_str()
            aria-hidden="true"
            class=class
        ></div>
    }
}
