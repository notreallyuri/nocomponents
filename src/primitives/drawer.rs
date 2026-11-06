//! A sheet you can throw away with your thumb.
//!
//! Modality, the focus trap, the layer stack and the exit animation are the dialog's; what this
//! adds is the drag. It travels outward only, a flick dismisses as surely as a haul, and a press
//! landing in something already scrolled belongs to that scroller.
//!
//! Given `snap_points` it also rests at more than one height: the panel is as tall as the highest
//! point and slides to whichever one is active, a release settles on the nearest (a flick moves
//! one point, a drag below `dismiss_below` throws it away), and the decision itself is the pure
//! [`settle`].

use crate::{
    primitives::{
        dialog::{DialogContentRoot, DialogRoot, use_dialog},
        drag::{DragEnd, DragPoint, use_drag},
    },
    utils::{
        controllable::{controllable, forward},
        types::Side,
    },
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::Element;

/// How far the panel must travel, as a fraction of its own length, before a release dismisses.
const DISTANCE_THRESHOLD: f64 = 0.25;
/// Pixels per millisecond past which a release counts as a flick regardless of distance.
const VELOCITY_THRESHOLD: f64 = 0.4;
/// How recently the pointer must have moved for a release to count as a flick at all.
const FLICK_WINDOW: f64 = 100.0;
/// How much a panel gives up, and how far it slides back, while a nested drawer sits over it.
const RECEDE_SCALE: f64 = 0.96;
const RECEDE_SHIFT: f64 = 14.0;
/// Depth past which the recede stops compounding; further back the panel would be a sliver.
const RECEDE_MAX_STEPS: usize = 3;

/// Below this share of the viewport, along the drawer's axis, a released snapping drawer is
/// dismissed.
pub const DEFAULT_DISMISS_BELOW: f64 = 0.22;
/// Pixels per millisecond past which a release on a snapping drawer moves one snap point.
pub const DEFAULT_SNAP_FLICK_VELOCITY: f64 = 0.6;

/// Where a released snapping drawer comes to rest.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Settle {
    /// Rest at this snap point.
    Snap(f64),
    Dismiss,
}

/// Decides where a released drawer goes.
///
/// - `position` is how much of the viewport the drawer covers along its axis at the release, as a
///   fraction: the same unit as `snap_points`, so `0.4` is a panel 40 percent of the way across.
/// - `velocity` is the release speed in pixels per millisecond, positive toward dismissal (the
///   panel shrinking) and zero for a release after a pause.
/// - `dismiss_below` is the position under which the drawer is thrown away however it got there.
/// - `flick_threshold` is the speed, in either direction, that counts as a flick.
///
/// Below `dismiss_below` it dismisses; a flick moves one snap point from `position` in its
/// direction, past the lowest dismissing and past the highest staying on it; anything slower goes
/// to the nearest. With no snap points there is a single rest at the full extent, `Snap(1.0)`.
pub fn settle(
    position: f64,
    velocity: f64,
    snap_points: &[f64],
    dismiss_below: f64,
    flick_threshold: f64,
) -> Settle {
    const EPSILON: f64 = 1e-9;

    let mut points: Vec<f64> = snap_points.to_vec();
    if points.is_empty() {
        points.push(1.0);
    }
    points.sort_by(|a, b| a.total_cmp(b));

    if position < dismiss_below {
        return Settle::Dismiss;
    }

    if velocity > flick_threshold {
        return match points
            .iter()
            .rev()
            .find(|point| **point < position - EPSILON)
        {
            Some(point) => Settle::Snap(*point),
            None => Settle::Dismiss,
        };
    }

    if velocity < -flick_threshold {
        return match points.iter().find(|point| **point > position + EPSILON) {
            Some(point) => Settle::Snap(*point),
            None => Settle::Snap(points[points.len() - 1]),
        };
    }

    let nearest = points
        .iter()
        .min_by(|a, b| (position - **a).abs().total_cmp(&(position - **b).abs()))
        .copied()
        .unwrap_or(1.0);
    Settle::Snap(nearest)
}

#[derive(Copy, Clone)]
pub struct DrawerContext {
    pub side: Side,
    /// The heights the drawer rests at, as fractions of the viewport along its axis. Empty means
    /// it is as tall as its content and has one resting place.
    pub snap_points: Signal<Vec<f64>>,
    /// The snap point the drawer is at, or `None` for the first.
    pub active_snap: RwSignal<Option<f64>>,
    pub dismiss_below: Signal<f64>,
    pub flick_velocity: Signal<f64>,
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
    /// The snap point the drawer rests at: the active one if it names a point, otherwise the
    /// first. `None` when there are no snap points.
    pub fn current_snap(&self) -> Option<f64> {
        self.snap_index()
            .map(|index| self.snap_points.with(|points| points[index]))
    }

    /// The position of [`Self::current_snap`] in `snap_points`.
    pub fn snap_index(&self) -> Option<usize> {
        let active = self.active_snap.get();
        self.snap_points.with(|points| {
            if points.is_empty() {
                return None;
            }
            match active {
                Some(active) => points
                    .iter()
                    .enumerate()
                    .min_by(|a, b| (active - a.1).abs().total_cmp(&(active - b.1).abs()))
                    .map(|(index, _)| index),
                None => Some(0),
            }
        })
    }

    fn highest_snap(&self) -> f64 {
        self.snap_points
            .with(|points| points.iter().copied().fold(0.0, f64::max))
    }

    /// How far below its tallest the panel rests, as a fraction of the viewport. Zero without
    /// snap points.
    fn snap_rest(&self) -> f64 {
        match self.current_snap() {
            Some(current) => (self.highest_snap() - current).max(0.0),
            None => 0.0,
        }
    }

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
        let rest = self.snap_rest();

        if offset == 0.0 && depth == 0 && rest == 0.0 {
            return String::new();
        }

        // Receding moves inward, the opposite way from a dismissal, and compounds with depth.
        let travel = if offset != 0.0 {
            offset
        } else {
            -RECEDE_SHIFT * depth as f64
        };

        let sign = match self.side {
            Side::Bottom | Side::Right => 1.0,
            Side::Top | Side::Left => -1.0,
        };
        let unit = if self.is_vertical() { "vh" } else { "vw" };
        let amount = if rest > 0.0 {
            format!("calc({}{unit} + {}px)", sign * rest * 100.0, sign * travel)
        } else {
            format!("{}px", sign * travel)
        };

        let translate = if self.is_vertical() {
            format!("translate3d(0, {amount}, 0)")
        } else {
            format!("translate3d({amount}, 0, 0)")
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
    /// Whether the drawer starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the drawer opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// Non-modal leaves the page usable underneath: no overlay, no focus trap, and an outside
    /// pointer closes the drawer.
    #[prop(default = true)]
    modal: bool,
    /// Heights the drawer rests at, as fractions of the viewport along its axis — `vec![0.4,
    /// 0.88]` is a peek and a full. The panel is as tall as the highest and slides to the active
    /// one. Empty is an ordinary drawer.
    #[prop(optional, into)]
    snap_points: Signal<Vec<f64>>,
    /// The snap point the drawer is at, held by the caller. `None` means the first. Omit it and
    /// the root keeps its own.
    #[prop(default = None, into)]
    active_snap: Option<RwSignal<Option<f64>>>,
    /// The snap point the drawer starts at. Ignored when `active_snap` is given.
    #[prop(default = None)]
    default_active_snap: Option<f64>,
    /// Called with the new snap point whenever it changes, and with `None` when a drag dismisses
    /// the drawer.
    #[prop(optional, into)]
    on_active_snap_change: Option<Callback<Option<f64>>>,
    /// The share of the viewport under which a released drawer is dismissed, however it got there.
    #[prop(default = DEFAULT_DISMISS_BELOW.into(), into)]
    dismiss_below: Signal<f64>,
    /// Release speed, in pixels per millisecond, that moves a snapping drawer one snap point in
    /// the flick's direction, or dismisses it from the lowest.
    #[prop(default = DEFAULT_SNAP_FLICK_VELOCITY.into(), into)]
    flick_velocity: Signal<f64>,
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
        snap_points,
        active_snap: controllable(active_snap, default_active_snap, on_active_snap_change),
        dismiss_below,
        flick_velocity,
        offset: RwSignal::new(0.0),
        dragging: RwSignal::new(false),
        nested_open: RwSignal::new(0),
        ancestors: StoredValue::new(ancestors),
    };

    view! {
        <DialogRoot open=open default_open=default_open on_open_change=forward(on_open_change) modal=modal>
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

    // The panel's length along the drag axis, measured at the press: what the travel is judged
    // against, and how far it has left to go once a release dismisses it.
    let length = StoredValue::new(0.0);
    // The viewport along the axis, measured at the press: what snap points are fractions of.
    let extent = StoredValue::new(0.0);

    let drag = use_drag(move |point: DragPoint| {
        let travel = ctx.travel(point.dx, point.dy);

        // Outward only, unless there are taller snap points to haul toward: past the highest the
        // panel stays put.
        let floor = match untrack(|| ctx.current_snap()) {
            Some(current) => -(untrack(|| ctx.highest_snap()) - current) * extent.get_value(),
            None => 0.0,
        };
        ctx.offset.set(travel.max(floor));
    })
    .on_end(move |end: DragEnd| {
        ctx.dragging.set(false);

        let travel = ctx.offset.get_untracked();
        let length = length.get_value();

        if let Some(current) = untrack(|| ctx.current_snap()) {
            let extent = extent.get_value();
            let velocity = if end.idle < FLICK_WINDOW {
                ctx.travel(end.velocity_x, end.velocity_y)
            } else {
                0.0
            };
            let position = if extent > 0.0 {
                current - travel / extent
            } else {
                current
            };
            let settled = ctx.snap_points.with_untracked(|points| {
                settle(
                    position,
                    velocity,
                    points,
                    ctx.dismiss_below.get_untracked(),
                    ctx.flick_velocity.get_untracked(),
                )
            });

            match settled {
                Settle::Snap(snap) => {
                    if (snap - current).abs() > f64::EPSILON {
                        ctx.active_snap.set(Some(snap));
                    }
                    ctx.offset.set(0.0);
                }
                Settle::Dismiss => {
                    dialog.close();
                    ctx.offset.set(length);
                    ctx.active_snap.set(None);
                }
            }
            return;
        }

        // A pause before release is a placement, not a throw; the velocity is projected onto the
        // drag axis, so a shove back toward the edge comes out negative and does not dismiss.
        let velocity = if end.idle < FLICK_WINDOW {
            ctx.travel(end.velocity_x, end.velocity_y)
        } else {
            0.0
        };

        let far_enough = length > 0.0 && travel / length > DISTANCE_THRESHOLD;
        let fast_enough = velocity > VELOCITY_THRESHOLD && travel > 0.0;

        if far_enough || fast_enough {
            dialog.close();
            // Carry on to closed rather than zeroing, which would rewind the panel before
            // sliding it out. The effect below clears the offset on the next open.
            ctx.offset.set(length);
        } else {
            ctx.offset.set(0.0);
        }
    });

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

        let rect = panel.get_bounding_client_rect();
        length.set_value(if ctx.is_vertical() {
            rect.height()
        } else {
            rect.width()
        });

        let viewport_extent = web_sys::window().and_then(|window| {
            if ctx.is_vertical() {
                window.inner_height().ok()
            } else {
                window.inner_width().ok()
            }
            .and_then(|value| value.as_f64())
        });
        extent.set_value(viewport_extent.unwrap_or(0.0));

        ctx.dragging.set(true);
        drag.start(&e);
    };

    view! {
        <div
            node_ref=panel_ref
            data-slot="drawer"
            data-side=ctx.side.as_str()
            data-dragging=move || ctx.dragging.get().then_some("true")
            data-snap=move || ctx.snap_index().map(|index| index.to_string())
            data-nested=move || (ctx.nested_open.get() > 0).then(|| ctx.nested_open.get().to_string())
            // The panel animates rather than the dialog content, so it carries `data-state`.
            data-state=move || if dialog.is_open.get() { "open" } else { "closed" }
            style=move || {
                let mut transform = match ctx.transform().as_str() {
                    "" => String::new(),
                    value => {
                        format!("transform: {value}; transform-origin: {};", ctx.transform_origin())
                    }
                };
                if ctx.highest_snap() > 0.0 {
                    let tallest = ctx.highest_snap() * 100.0;
                    transform.push_str(
                        &if ctx.is_vertical() {
                            format!(" height: {tallest}vh; max-height: none;")
                        } else {
                            format!(" width: {tallest}vw; max-width: none;")
                        },
                    );
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: [f64; 2] = [0.40, 0.88];
    const BELOW: f64 = 0.22;
    const FLICK: f64 = 0.6;

    fn settled(position: f64, velocity: f64) -> Settle {
        settle(position, velocity, &POINTS, BELOW, FLICK)
    }

    #[test]
    fn a_slow_release_goes_to_the_nearest_snap() {
        assert_eq!(settled(0.45, 0.0), Settle::Snap(0.40));
        assert_eq!(settled(0.70, 0.0), Settle::Snap(0.88));
        assert_eq!(settled(0.64, 0.0), Settle::Snap(0.40));
        assert_eq!(settled(0.65, 0.1), Settle::Snap(0.88));
        assert_eq!(settled(0.95, -0.2), Settle::Snap(0.88));
    }

    #[test]
    fn a_slow_release_between_the_floor_and_the_lowest_snap_returns_to_it() {
        assert_eq!(settled(0.25, 0.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_flick_up_moves_one_snap() {
        assert_eq!(settled(0.42, -1.0), Settle::Snap(0.88));
        assert_eq!(settled(0.40, -0.7), Settle::Snap(0.88));
    }

    #[test]
    fn a_flick_up_from_the_highest_stays_on_it() {
        assert_eq!(settled(0.88, -2.0), Settle::Snap(0.88));
        assert_eq!(settled(0.90, -2.0), Settle::Snap(0.88));
    }

    #[test]
    fn a_flick_down_moves_one_snap() {
        assert_eq!(settled(0.86, 1.0), Settle::Snap(0.40));
        assert_eq!(settled(0.88, 0.7), Settle::Snap(0.40));
        assert_eq!(settled(0.45, 1.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_flick_down_from_the_lowest_dismisses() {
        assert_eq!(settled(0.40, 1.0), Settle::Dismiss);
        assert_eq!(settled(0.35, 0.9), Settle::Dismiss);
    }

    #[test]
    fn a_release_below_the_floor_dismisses_however_slowly() {
        assert_eq!(settled(0.21, 0.0), Settle::Dismiss);
        assert_eq!(settled(0.10, -0.1), Settle::Dismiss);
        assert_eq!(settled(0.21, -5.0), Settle::Dismiss);
        assert_eq!(settled(0.22, 0.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_speed_at_the_threshold_is_not_yet_a_flick() {
        assert_eq!(settled(0.45, FLICK), Settle::Snap(0.40));
        assert_eq!(settled(0.70, -FLICK), Settle::Snap(0.88));
    }

    #[test]
    fn the_order_the_points_are_given_in_does_not_matter() {
        let reversed = [0.88, 0.40];
        assert_eq!(
            settle(0.42, -1.0, &reversed, BELOW, FLICK),
            Settle::Snap(0.88)
        );
        assert_eq!(settle(0.40, 1.0, &reversed, BELOW, FLICK), Settle::Dismiss);
    }

    #[test]
    fn a_single_snap_is_a_rest_that_a_flick_down_dismisses() {
        assert_eq!(settle(0.6, 0.0, &[0.6], BELOW, FLICK), Settle::Snap(0.6));
        assert_eq!(settle(0.6, 0.9, &[0.6], BELOW, FLICK), Settle::Dismiss);
        assert_eq!(settle(0.6, -0.9, &[0.6], BELOW, FLICK), Settle::Snap(0.6));
    }

    #[test]
    fn no_snap_points_is_one_rest_at_the_full_extent() {
        assert_eq!(settle(0.97, 0.0, &[], BELOW, FLICK), Settle::Snap(1.0));
        assert_eq!(settle(0.5, 0.0, &[], BELOW, FLICK), Settle::Snap(1.0));
        assert_eq!(settle(0.5, 0.9, &[], BELOW, FLICK), Settle::Dismiss);
        assert_eq!(settle(0.1, 0.0, &[], BELOW, FLICK), Settle::Dismiss);
    }
}
