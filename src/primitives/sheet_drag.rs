//! The drag of a layer shown as a phone sheet.
//!
//! [`crate::utils::sheet`] decides *whether* a layer is a sheet and draws its header; this is what
//! the header does when it is pressed. Dragging the grabber row down moves the whole panel with the
//! pointer, and on release the panel either springs back or is dismissed — the same decision the
//! drawer makes, [`settle_drag`], so a flick and a haul mean the same thing in both.
//!
//! Only the header grabs. The body keeps its scrolling, so a list that is being scrolled can never
//! pull the sheet down with it.
//!
//! Given `snap_points` (fractions of the viewport height, like the drawer's) the sheet is as tall
//! as the highest point and rests at the active one: it opens at the first, a drag settles on the
//! nearest, a flick moves one point, and a release below the lowest dismisses.

use crate::utils::sheet::SheetHeader;
use crate::{
    primitives::drag::{Drag, DragEnd, DragPoint, use_drag},
    utils::settle::{
        DEFAULT_DISMISS_BELOW, DEFAULT_SNAP_FLICK_VELOCITY, FLICK_WINDOW, Release, Settle,
        settle_drag,
    },
};
use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use web_sys::Element;

/// What a press on these should leave alone: it is a click, not a grab.
const INTERACTIVE: &str = "button, a, input, textarea, select, [role='button'], [contenteditable]";

/// The drag state of one sheet. `Copy`, so the header and the panel can each hold it.
#[derive(Copy, Clone)]
pub struct SheetGrab {
    sheet: Signal<bool>,
    draggable: Signal<bool>,
    snap_points: Signal<Vec<f64>>,
    active: RwSignal<Option<f64>>,
    offset: RwSignal<f64>,
    drag: Drag,
    length: StoredValue<f64>,
    extent: StoredValue<f64>,
}

/// Wires the drag of a sheet.
///
/// `sheet` is [`crate::utils::sheet::use_sheet_presentation`]'s answer, `open` whether the layer
/// is open — it puts the panel back at rest each time it opens — and `close` is what a dismissing
/// release calls; if the layer is still open a frame later (a controlled `open` that refused), the
/// panel goes back to rest. `snap_points` empty is a sheet as tall as its content, and
/// `draggable` off makes the header inert.
///
/// Put [`SheetGrab::style`], [`SheetGrab::dragging`] and [`SheetGrab::snap`] on the panel, and
/// hand [`SheetGrab::on_grab`] to the header.
pub fn use_sheet_grab(
    sheet: Signal<bool>,
    open: Signal<bool>,
    snap_points: Signal<Vec<f64>>,
    draggable: Signal<bool>,
    close: Callback<()>,
) -> SheetGrab {
    let active = RwSignal::new(None::<f64>);
    let offset = RwSignal::new(0.0);
    let length = StoredValue::new(0.0);
    let extent = StoredValue::new(0.0);

    let current = move || {
        let active = active.get_untracked();
        snap_points.with_untracked(|points| {
            if points.is_empty() {
                return None;
            }
            Some(match active {
                Some(active) => points
                    .iter()
                    .copied()
                    .min_by(|a, b| (active - a).abs().total_cmp(&(active - b).abs()))
                    .unwrap_or(points[0]),
                None => points[0],
            })
        })
    };

    let drag = use_drag(move |point: DragPoint| {
        let floor = match current() {
            Some(current) => {
                let highest =
                    snap_points.with_untracked(|points| points.iter().copied().fold(0.0, f64::max));
                -(highest - current).max(0.0) * extent.get_value()
            }
            None => 0.0,
        };
        offset.set(point.dy.max(floor));
    })
    .on_end(move |end: DragEnd| {
        let travel = offset.get_untracked();
        let velocity = if end.idle < FLICK_WINDOW {
            end.velocity_y
        } else {
            0.0
        };
        let snapping = snap_points.with_untracked(|points| !points.is_empty());
        let current = current();

        let settled = snap_points.with_untracked(|points| {
            settle_drag(
                Release {
                    travel,
                    length: length.get_value(),
                    extent: extent.get_value(),
                    current,
                    velocity,
                },
                points,
                DEFAULT_DISMISS_BELOW,
                DEFAULT_SNAP_FLICK_VELOCITY,
            )
        });

        match settled {
            Settle::Snap(snap) => {
                if snapping && current.is_none_or(|current| (snap - current).abs() > f64::EPSILON) {
                    active.set(Some(snap));
                }
                offset.set(0.0);
            }
            // Left where it is: the exit animation carries on from the pointer, rather than
            // rewinding the panel before it slides out.
            Settle::Dismiss => {
                close.run(());
                request_animation_frame(move || {
                    if open.get_untracked() {
                        offset.set(0.0);
                    }
                });
            }
        }
    });

    Effect::new(move |_| {
        if open.get() {
            offset.set(0.0);
            active.set(None);
        }
    });

    SheetGrab {
        sheet,
        draggable,
        snap_points,
        active,
        offset,
        drag,
        length,
        extent,
    }
}

impl SheetGrab {
    /// Whether the header can be dragged at all.
    pub fn draggable(&self) -> Signal<bool> {
        self.draggable
    }

    /// Whether a drag is in progress. Written as `data-dragging="true"` so the styled layer can
    /// turn its transition off for exactly that long.
    pub fn dragging(&self) -> Option<&'static str> {
        self.drag.active.get().then_some("true")
    }

    /// The position in `snap_points` the sheet rests at, as `data-snap`. Nothing without snap
    /// points or outside a sheet.
    pub fn snap(&self) -> Option<String> {
        if !self.sheet.get() {
            return None;
        }
        let active = self.active.get();
        self.snap_points.with(|points| {
            if points.is_empty() {
                return None;
            }
            let index = match active {
                Some(active) => points
                    .iter()
                    .enumerate()
                    .min_by(|a, b| (active - a.1).abs().total_cmp(&(active - b.1).abs()))
                    .map(|(index, _)| index)
                    .unwrap_or(0),
                None => 0,
            };
            Some(index.to_string())
        })
    }

    fn highest(&self) -> f64 {
        self.snap_points
            .with(|points| points.iter().copied().fold(0.0, f64::max))
    }

    fn rest(&self) -> f64 {
        let highest = self.highest();
        let active = self.active.get();
        let current = self.snap_points.with(|points| {
            points.first().map(|first| match active {
                Some(active) => points
                    .iter()
                    .copied()
                    .min_by(|a, b| (active - a).abs().total_cmp(&(active - b).abs()))
                    .unwrap_or(*first),
                None => *first,
            })
        });
        current.map_or(0.0, |current| (highest - current).max(0.0))
    }

    /// The panel's inline geometry: where the drag has put it, and the height snap points give it.
    /// Empty outside a sheet, so the panel is untouched at `md` and up.
    pub fn style(&self) -> String {
        if !self.sheet.get() {
            return String::new();
        }

        let offset = self.offset.get();
        let rest = self.rest();
        let mut style = String::new();

        if offset != 0.0 || rest > 0.0 {
            let amount = if rest > 0.0 {
                format!("calc({}dvh + {offset}px)", rest * 100.0)
            } else {
                format!("{offset}px")
            };
            style.push_str(&format!("transform: translate3d(0, {amount}, 0);"));
        }

        let highest = self.highest();
        if highest > 0.0 {
            style.push_str(&format!(
                " height: {}dvh; max-height: none;",
                highest * 100.0
            ));
        }

        // The panel has to track the pointer exactly; easing would read as lag.
        if self.drag.active.get() {
            style.push_str(" transition: none;");
        }

        style
    }

    /// The handler for a press on the header.
    pub fn on_grab(&self) -> Callback<ev::PointerEvent> {
        let grab = *self;
        Callback::new(move |e: ev::PointerEvent| grab.press(&e))
    }

    fn press(&self, e: &ev::PointerEvent) {
        if !self.sheet.get_untracked()
            || !self.draggable.get_untracked()
            || (e.pointer_type() == "mouse" && e.button() != 0)
        {
            return;
        }

        if let Some(target) = e.target()
            && let Ok(target) = target.dyn_into::<Element>()
            && matches!(target.closest(INTERACTIVE), Ok(Some(_)))
        {
            return;
        }

        let panel = e
            .current_target()
            .and_then(|header| header.dyn_into::<Element>().ok())
            .and_then(|header| header.parent_element());
        let Some(panel) = panel else {
            return;
        };

        self.length
            .set_value(panel.get_bounding_client_rect().height());
        let viewport = window()
            .visual_viewport()
            .map(|viewport| viewport.height())
            .or_else(|| {
                window()
                    .inner_height()
                    .ok()
                    .and_then(|height| height.as_f64())
            });
        self.extent.set_value(viewport.unwrap_or(0.0));

        self.drag.start(e);
    }
}

/// The sheet's panel for a layer whose content is a `role="menu"` or `role="listbox"` element.
///
/// Those roles permit only items inside them, and a title and a close button are not items, so the
/// header cannot be a child of the element that carries the role. In a sheet the role element
/// therefore goes *inside* this frame, beside the header, and the frame is what the styled layer's
/// classes, the drag transform and the `data-*` marks land on. A floating layer has no frame.
#[component]
pub fn SheetFrame(
    grab: SheetGrab,
    sheet: Signal<bool>,
    open: Signal<bool>,
    close: Callback<()>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] title: Signal<String>,
    #[prop(default = true.into(), into)] show_close: Signal<bool>,
    #[prop(default = "Close".into(), into)] close_label: Signal<String>,
    #[prop(default = None, into)] close_icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="menu-sheet"
            data-presentation="sheet"
            data-state=move || if open.get() { "open" } else { "closed" }
            data-dragging=move || grab.dragging()
            data-snap=move || grab.snap()
            style=move || grab.style()
            class=class
        >
            <SheetHeader
                sheet=sheet
                title=title
                on_grab=grab.on_grab()
                draggable=grab.draggable()
                close=close
                show_close=show_close
                close_label=close_label
                close_icon=close_icon
            />
            {children()}
        </div>
    }
}
