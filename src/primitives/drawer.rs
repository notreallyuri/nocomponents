//! A sheet you can throw away with your thumb.
//!
//! Modality, the focus trap, the layer stack and the exit animation are the dialog's; what this
//! adds is the drag. It travels outward only, a flick dismisses as surely as a haul, and a press
//! landing in something already scrolled belongs to that scroller.

use crate::{
    primitives::dialog::{DialogContentRoot, DialogRoot, use_dialog},
    utils::types::Side,
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::Element;

/// How far the panel must travel, as a fraction of its own length, before a release dismisses.
const DISTANCE_THRESHOLD: f64 = 0.25;
/// Pixels per millisecond past which a release counts as a flick regardless of distance.
const VELOCITY_THRESHOLD: f64 = 0.4;
/// How much a panel gives up, and how far it slides back, while a nested drawer sits over it.
const RECEDE_SCALE: f64 = 0.96;
const RECEDE_SHIFT: f64 = 14.0;
/// Depth past which the recede stops compounding; further back the panel would be a sliver.
const RECEDE_MAX_STEPS: usize = 3;

#[derive(Copy, Clone)]
pub struct DrawerContext {
    pub side: Side,
    /// Drag distance from the resting place, in pixels. Never negative: it travels outward only.
    pub offset: RwSignal<f64>,
    pub dragging: RwSignal<bool>,
    /// Drawers open inside this one, at any depth. A count rather than a flag: two can overlap
    /// while one animates out, and the recede compounds with it.
    pub nested_open: RwSignal<usize>,
    /// The counters of every drawer this one sits inside, outermost first; opening announces
    /// itself to all of them. Signals rather than contexts, so `DrawerContext` stays `Copy`.
    ancestors: StoredValue<Vec<RwSignal<usize>>>,
}

impl DrawerContext {
    /// Whether a nested drawer is currently sitting over this one.
    pub fn is_receded(&self) -> bool {
        self.nested_open.get() > 0
    }

    /// How many steps back this panel sits, capped at `RECEDE_MAX_STEPS`.
    pub fn recede_depth(&self) -> usize {
        self.nested_open.get().min(RECEDE_MAX_STEPS)
    }

    /// The panel's transform: its drag offset, or its recede when a nested drawer is over it.
    /// A drag in progress wins — the panel under the finger has to follow the finger.
    pub fn transform(&self) -> String {
        let offset = self.offset.get();
        let depth = self.recede_depth();

        // Receding moves inward, the opposite way from a dismissal, and compounds with depth.
        let travel = match (offset != 0.0, depth > 0) {
            (true, _) => offset,
            (false, true) => -RECEDE_SHIFT * depth as f64,
            (false, false) => return String::new(),
        };

        let translate = match self.side {
            Side::Bottom => format!("translate3d(0, {travel}px, 0)"),
            Side::Top => format!("translate3d(0, {}px, 0)", -travel),
            Side::Right => format!("translate3d({travel}px, 0, 0)"),
            Side::Left => format!("translate3d({}px, 0, 0)", -travel),
        };

        if depth > 0 && offset == 0.0 {
            format!("{translate} scale({})", RECEDE_SCALE.powi(depth as i32))
        } else {
            translate
        }
    }

    /// Which corner the recede shrinks toward: the edge facing the viewport, never the anchored
    /// one. Scaling about the centre would cancel the shift out and leave nothing peeking past.
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
    /// Non-modal leaves the page usable underneath: no overlay, no focus trap, and an outside
    /// pointer closes the drawer.
    #[prop(default = true)]
    modal: bool,
    children: Children,
) -> impl IntoView {
    // Read before providing, so this resolves to the parent: its chain plus itself becomes ours.
    let ancestors = match use_context::<DrawerContext>() {
        Some(parent) => {
            let mut chain = parent.ancestors.get_value();
            chain.push(parent.nested_open);
            chain
        }
        None => Vec::new(),
    };

    let context = DrawerContext {
        side,
        offset: RwSignal::new(0.0),
        dragging: RwSignal::new(false),
        nested_open: RwSignal::new(0),
        ancestors: StoredValue::new(ancestors),
    };

    view! {
        <DialogRoot open=open modal=modal>
            <Provider value=context>{children()}</Provider>
        </DialogRoot>
    }
}

/// Anything that owns the press already: a press on these that drifts is a click, not a drag.
const INTERACTIVE: &str = "button, a, input, textarea, select, [role='button'], [contenteditable]";

/// Whether the press landed in something already scrolled, in which case the gesture is its.
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

    // A drawer closed some other way mid-drag must not reopen still shoved off its edge.
    Effect::new(move |_| {
        if dialog.is_open.get() {
            ctx.offset.set(0.0);
        }
    });

    // Nudged on transitions rather than assigned: siblings each own a share of the same count.
    let counted = StoredValue::new(false);
    let release_ancestors = move || {
        if counted.get_value() {
            counted.set_value(false);
            ctx.ancestors.with_value(|chain| {
                for counter in chain {
                    counter.update(|open| *open = open.saturating_sub(1));
                }
            });
        }
    };

    Effect::new(move |_| {
        let is_open = dialog.is_open.get();

        if is_open && !counted.get_value() {
            counted.set_value(true);
            ctx.ancestors.with_value(|chain| {
                for counter in chain {
                    counter.update(|open| *open += 1);
                }
            });
        } else if !is_open {
            release_ancestors();
        }
    });

    // Torn down while open, it still has to hand the count back.
    on_cleanup(release_ancestors);

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

        // Velocity of the last movement, sampled between moves: at release it always reads zero.
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

            // A pause before release is a placement, not a throw; a signed velocity means a
            // shove back toward the edge does not dismiss.
            let velocity = if js_sys::Date::now() - last_time < 100.0 {
                last_velocity.get_value()
            } else {
                0.0
            };

            let far_enough = length > 0.0 && travel / length > DISTANCE_THRESHOLD;
            let fast_enough = velocity > VELOCITY_THRESHOLD && travel > 0.0;

            if far_enough || fast_enough {
                dialog.close();
                // Carry on to closed rather than zeroing, which would rewind the panel before
                // sliding it out. The effect above clears the offset on the next open.
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
            data-nested=move || (ctx.nested_open.get() > 0).then(|| ctx.nested_open.get().to_string())
            // The panel animates rather than the dialog content, so it carries `data-state`.
            data-state=move || if dialog.is_open.get() { "open" } else { "closed" }
            style=move || {
                let transform = match ctx.transform().as_str() {
                    "" => String::new(),
                    value => {
                        format!("transform: {value}; transform-origin: {};", ctx.transform_origin())
                    }
                };
                // The panel has to track the pointer exactly; easing would read as lag.
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

/// The grab bar. An affordance only, since the whole panel is draggable.
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
