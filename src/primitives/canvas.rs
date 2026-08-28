//! An infinite surface you can pan and zoom, with things placed on it in world coordinates.
//!
//! The arithmetic lives in [`crate::utils::viewport`], which knows nothing about the DOM and is
//! unit-tested; this file reads the boxes, listens to the gestures, and writes one transform.
//!
//! **The canvas does not own what is on it.** Nodes take their world position as props and the
//! caller keeps the list, the same stance as the kanban board reporting a move rather than
//! applying it. What this owns is where you are *looking*.
//!
//! Two things about the gestures are worth knowing before changing them. A browser reports a
//! trackpad **pinch as a wheel event with `ctrlKey` set** — there is no pinch event on the
//! desktop — so that flag, not a modifier the reader pressed, is what separates zooming from
//! two-finger panning. And a wheel notch means a *ratio* rather than an amount: two notches in
//! should cover the same ground however far in you already are, which is why the zoom multiplies.
//!
//! **A press on the surface is a click, not a pan.** Every board worth the name has something to
//! click *on* — a node to select, a spot to put one — and a surface that swallows the primary
//! button leaves the caller nothing to build that from. Panning is space and drag, the middle
//! button, or two fingers, which is what Figma, Miro and every editor since have settled on;
//! `pan_on_drag` turns the hand tool back on for a board that really is only scenery.

use crate::{
    primitives::drag::use_drag,
    utils::viewport::{BoxHandle, Viewport, WorldBox, ZoomLimits},
};
use leptos::wasm_bindgen::JsCast;
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlTextAreaElement};

/// How much of a wheel delta becomes zoom. Small, because a trackpad sends a lot of them.
const WHEEL_ZOOM: f64 = 0.002;
/// A line of wheel delta in pixels, for the mice that report in lines rather than pixels.
const LINE_HEIGHT: f64 = 16.0;
/// What the zoom buttons do per press.
const STEP: f64 = 1.25;
/// How far a pointer may travel between press and release and still count as a click. A press
/// that panned the board is not the reader asking for something at the place it ended up.
const CLICK_SLOP: f64 = 4.0;
/// How big a resize grip is **on screen**, in pixels.
///
/// Divided by the zoom before it is drawn, because the grips live inside the transformed layer:
/// left alone, a 10px square is 1px at 0.1× — too small to hit — and 40px at 4×, where it covers
/// the node it belongs to. Everything else on the board is meant to scale; a control is not.
const HANDLE_SIZE: f64 = 10.0;

/// What a gesture on a node did to it.
///
/// One shape for a move and for a resize, and one callback for both, because the thing a caller
/// does with either is the same: write an operation somebody else can replay. `handle` is what
/// tells them apart when it matters — `None` is a move, `Some` is the grip that did the resizing.
///
/// It carries where the gesture *began* as well as where it ended, which a caller cannot keep for
/// itself: the live callbacks have been overwriting that position forty times a second since the
/// press. An operation a peer can invert needs both ends of it, and so does an undo stack.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CanvasGesture {
    pub from: WorldBox,
    pub to: WorldBox,
    pub handle: Option<BoxHandle>,
}

/// A point on the board, in world coordinates.
///
/// A named struct rather than `(f64, f64)`, so a callback that one day wants the modifiers or the
/// button takes a field instead of breaking every caller.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CanvasPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct CanvasContext {
    pub viewport: RwSignal<Viewport>,
    pub limits: ZoomLimits,
    /// The clipping surface. Gestures are measured against its box, never the window's.
    pub surface_ref: AnyNodeRef,
    /// True while the surface is being dragged, so a cursor can say so.
    pub panning: RwSignal<bool>,
    /// True while space is held, which turns a press anywhere into a pan.
    pub space_held: RwSignal<bool>,
}

impl CanvasContext {
    /// The surface's own box, which every screen coordinate here is relative to.
    fn surface(&self) -> Option<web_sys::DomRect> {
        Some(self.surface_ref.get_untracked()?.get_bounding_client_rect())
    }

    /// A pointer's position in the surface's own coordinates.
    ///
    /// Client coordinates are relative to the window, and the surface is almost never at its
    /// corner — zooming toward an uncorrected pointer drifts by however far down the page the
    /// canvas sits.
    pub fn to_surface(&self, client_x: f64, client_y: f64) -> Option<(f64, f64)> {
        let rect = self.surface()?;
        Some((client_x - rect.left(), client_y - rect.top()))
    }

    /// What is under a pointer, in world coordinates.
    pub fn world_at(&self, client_x: f64, client_y: f64) -> Option<(f64, f64)> {
        let (x, y) = self.to_surface(client_x, client_y)?;
        Some(self.viewport.get_untracked().screen_to_world(x, y))
    }

    pub fn pan_by(&self, dx: f64, dy: f64) {
        self.viewport
            .update(|viewport| *viewport = viewport.panned(dx, dy));
    }

    /// Zooms about a point given in *client* coordinates.
    pub fn zoom_by_at(&self, factor: f64, client_x: f64, client_y: f64) {
        let Some((x, y)) = self.to_surface(client_x, client_y) else {
            return;
        };
        let limits = self.limits;
        self.viewport
            .update(|viewport| *viewport = viewport.zoomed_by(factor, x, y, limits));
    }

    /// Zooms about the middle of the surface, which is what a button press means — there is no
    /// pointer to zoom toward.
    pub fn zoom_by(&self, factor: f64) {
        let Some(rect) = self.surface() else {
            return;
        };
        let limits = self.limits;
        let (x, y) = (rect.width() / 2.0, rect.height() / 2.0);
        self.viewport
            .update(|viewport| *viewport = viewport.zoomed_by(factor, x, y, limits));
    }

    pub fn zoom_in(&self) {
        self.zoom_by(STEP);
    }

    pub fn zoom_out(&self) {
        self.zoom_by(1.0 / STEP);
    }

    /// Brings a box of world into view, with room to spare around it.
    pub fn fit(&self, content: WorldBox, padding: f64) {
        let Some(rect) = self.surface() else {
            return;
        };
        self.viewport.set(Viewport::fitted(
            content,
            rect.width(),
            rect.height(),
            padding,
            self.limits,
        ));
    }

    pub fn reset(&self) {
        self.viewport.set(Viewport::default());
    }

    /// The box that contains everything on the canvas, in world coordinates.
    ///
    /// Read out of the DOM — every `[data-slot="canvas-node"]` under the surface — rather than
    /// from a registry the nodes keep in step. The caller owns the node list, so a registry here
    /// would be a second copy that disagrees the moment one is added; this way nodes can come and
    /// go without telling anybody, which is the same trade the kanban board makes.
    ///
    /// `None` for an empty canvas, because "fit nothing" has no answer worth acting on.
    pub fn content_bounds(&self) -> Option<WorldBox> {
        let surface = self.surface_ref.get_untracked()?;
        let origin = surface.get_bounding_client_rect();
        let nodes = surface.query_selector_all("[data-slot=canvas-node]").ok()?;
        let viewport = self.viewport.get_untracked();

        if viewport.zoom == 0.0 {
            return None;
        }

        WorldBox::around((0..nodes.length()).filter_map(|i| {
            let node = nodes.item(i)?.dyn_into::<Element>().ok()?;
            let rect = node.get_bounding_client_rect();
            // The rects come back already scaled by the transform, so the size divides back out.
            let (x, y) =
                viewport.screen_to_world(rect.left() - origin.left(), rect.top() - origin.top());
            Some(WorldBox::new(
                x,
                y,
                rect.width() / viewport.zoom,
                rect.height() / viewport.zoom,
            ))
        }))
    }

    /// Brings everything on the canvas into view. Does nothing at all when there is nothing.
    pub fn fit_content(&self, padding: f64) {
        if let Some(bounds) = self.content_bounds() {
            self.fit(bounds, padding);
        }
    }

    /// The transform the content layer wears. `transform-origin` is the top left, so this reads
    /// exactly like the arithmetic: scale about the origin, then translate.
    pub fn transform(&self) -> String {
        let viewport = self.viewport.get();
        format!(
            "transform: translate({}px, {}px) scale({}); transform-origin: 0 0;",
            viewport.x, viewport.y, viewport.zoom
        )
    }
}

pub fn use_canvas() -> CanvasContext {
    expect_context::<CanvasContext>()
}

#[component]
pub fn CanvasRoot(
    /// The viewport, for a caller that wants to read where the reader is or drive it elsewhere.
    #[prop(optional, into)]
    viewport: RwSignal<Viewport>,
    #[prop(optional)] limits: ZoomLimits,
    /// Whether dragging the surface with the primary button pans it — the hand tool, always on.
    ///
    /// Off by default, because a press is the one gesture a caller cannot get back: space and
    /// drag, the middle button and two fingers all still pan, and none of them is what anybody
    /// clicks a board with. Turn it on for a board that is only ever looked at.
    #[prop(default = false)]
    pan_on_drag: bool,
    /// Run for a click that landed on the board itself rather than on anything placed on it, and
    /// that did not travel far enough to have been a pan.
    ///
    /// The world coordinate is the part the caller cannot work out for itself — it needs the
    /// surface's own box and the current transform, both of which live in here.
    #[prop(default = None, into)]
    on_surface_click: Option<Callback<CanvasPoint>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let surface_ref = AnyNodeRef::new();
    let ctx = CanvasContext {
        viewport,
        limits,
        surface_ref,
        panning: RwSignal::new(false),
        space_held: RwSignal::new(false),
    };

    // `DragPoint` reports distance from the *press*, and panning wants distance since the last
    // move — panning by the cumulative figure every time accelerates the canvas away from the
    // pointer instead of following it.
    let travelled = StoredValue::new((0.0f64, 0.0f64));

    // Where the press landed, so the release can tell a click from a pan that happened to end
    // over something. Kept in client coordinates: the surface may have moved under the gesture.
    let pressed_at = StoredValue::new(None::<(f64, f64)>);

    let drag = use_drag(move |point| {
        let (last_x, last_y) = travelled.get_value();
        ctx.pan_by(point.dx - last_x, point.dy - last_y);
        travelled.set_value((point.dx, point.dy));
    })
    .on_start(move |_| travelled.set_value((0.0, 0.0)))
    // The release is what ends a pan, and it can land anywhere — including outside the surface,
    // where the surface's own `pointerup` never hears about it.
    .on_end(move |_| {
        travelled.set_value((0.0, 0.0));
        ctx.panning.set(false);
    });

    view! {
        <Provider value=ctx>
            <div
                node_ref=surface_ref
                data-slot="canvas"
                // The zoom, for the CSS that has to divide by it. A border or an outline inside
                // the transformed layer is scaled along with everything else, and there is no
                // class that says "one pixel however far in we are" — but `calc(1px / var(…))`
                // does, and this is the only place that knows the number.
                style=move || format!("--canvas-zoom: {};", viewport.get().zoom)
                // One attribute for what the pan gesture is doing, rather than one per flag:
                // `ready` and `active` overlap while space is held through a drag, and two
                // attributes styled separately leave which cursor wins to stylesheet order.
                data-pan=move || {
                    if ctx.panning.get() {
                        Some("active")
                    } else if pan_on_drag || ctx.space_held.get() {
                        Some("ready")
                    } else {
                        None
                    }
                }
                // A surface has to be focusable for the space key to reach it, and a canvas
                // nobody can tab to is a canvas the keyboard cannot pan.
                tabindex="0"
                on:wheel=move |e: ev::WheelEvent| {
                    // Without this the page zooms instead, or scrolls out from under the canvas.
                    e.prevent_default();
                    let scale = if e.delta_mode() == 1 { LINE_HEIGHT } else { 1.0 };
                    let (dx, dy) = (e.delta_x() * scale, e.delta_y() * scale);
                    // A pinch on a trackpad arrives here as ctrl+wheel; there is no pinch event.
                    if e.ctrl_key() || e.meta_key() {
                        let factor = (-dy * WHEEL_ZOOM).exp();
                        ctx.zoom_by_at(factor, e.client_x() as f64, e.client_y() as f64);
                    } else {
                        ctx.pan_by(-dx, -dy);
                    }
                }
                on:keydown=move |e: ev::KeyboardEvent| {
                    match e.key().as_str() {
                        " " => {
                            e.prevent_default();
                            ctx.space_held.set(true);
                        }
                        "0" if e.ctrl_key() || e.meta_key() => {
                            e.prevent_default();
                            ctx.reset();
                        }
                        "=" | "+" if e.ctrl_key() || e.meta_key() => {
                            e.prevent_default();
                            ctx.zoom_in();
                        }
                        "-" if e.ctrl_key() || e.meta_key() => {
                            e.prevent_default();
                            ctx.zoom_out();
                        }
                        _ => {}
                    }
                }
                on:keyup=move |e: ev::KeyboardEvent| {
                    if e.key() == " " {
                        ctx.space_held.set(false);
                    }
                }
                // Space stops being held while the surface is not looking, and a canvas that
                // believes otherwise pans on the next click.
                on:blur=move |_| ctx.space_held.set(false)
                on:pointerdown=move |e: ev::PointerEvent| {
                    pressed_at.set_value(Some((e.client_x() as f64, e.client_y() as f64)));
                    let middle = e.button() == 1;
                    let primary = e.button() == 0;
                    let with_space = primary && ctx.space_held.get_untracked();
                    if !(middle || with_space || (primary && pan_on_drag)) {
                        // Left alone, so the press focuses the surface the way a press on any
                        // other tabbable element does — which is what the space key needs.
                        return;
                    }
                    e.prevent_default();
                    ctx.panning.set(true);
                    drag.start(&e);
                }
                on:pointerup=move |_| ctx.panning.set(false)
                on:pointercancel=move |_| ctx.panning.set(false)
                on:click=move |e: ev::MouseEvent| {
                    let Some(on_surface_click) = on_surface_click else {
                        return;
                    };
                    if let Some((x, y)) = pressed_at.get_value()
                        && (e.client_x() as f64 - x).hypot(e.client_y() as f64 - y) > CLICK_SLOP
                    {
                        return;
                    }
                    // A click that landed on something the caller put on the board belongs to
                    // that thing. `closest` rather than the target itself, because a node's own
                    // markup is what the pointer actually hits.
                    if e
                        .target()
                        .and_then(|target| target.dyn_into::<Element>().ok())
                        .and_then(|target| target.closest("[data-slot=canvas-node]").ok().flatten())
                        .is_some()
                    {
                        return;
                    }
                    if let Some((x, y)) = ctx.world_at(e.client_x() as f64, e.client_y() as f64) {
                        on_surface_click.run(CanvasPoint { x, y });
                    }
                }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The layer everything on the canvas lives in, and the only thing that is transformed.
///
/// One transform for the whole world rather than one per node: the browser composites a single
/// layer, and a node's own `left`/`top` stay in world units where the caller wrote them.
#[component]
pub fn CanvasViewportRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_canvas();

    view! {
        <div data-slot="canvas-viewport" style=move || ctx.transform() class=class>
            {children()}
        </div>
    }
}

/// The dotted paper behind everything.
///
/// Outside the transformed layer on purpose. Inside it the dots would scale with the zoom, so
/// they would blur out at 4× and vanish at 0.1×; a repeating background whose *spacing* follows
/// the zoom and whose dots do not is what paper actually looks like.
#[component]
pub fn CanvasBackgroundRoot(
    /// World units between dots.
    #[prop(default = 24.0)]
    gap: f64,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_canvas();

    view! {
        <div
            data-slot="canvas-background"
            aria-hidden="true"
            style=move || {
                let viewport = ctx.viewport.get();
                let spacing = gap * viewport.zoom;
                format!(
                    "background-size: {spacing}px {spacing}px; background-position: {}px {}px;",
                    viewport.x,
                    viewport.y,
                )
            }
            class=class
        />
    }
}

/// One thing on the canvas, placed in world coordinates.
///
/// Position is a prop rather than state here: the caller keeps the list, so a node moved by
/// anything other than a drag still lands where the caller says it does. What the node owns is
/// the *gesture* — it reports where a drag would put it and the caller decides, the same stance
/// the kanban board takes with `on_move`.
///
/// The arithmetic a caller would otherwise have to write twice lives here: a pointer travels in
/// screen pixels and a node lives in world units, so every delta is divided by the zoom, and the
/// division needs the canvas' context, which only something inside the canvas can ask for.
#[component]
pub fn CanvasNodeRoot(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    /// World width and height. Left unset the node is sized by its content, which is what a
    /// label or a sticky wants — and what cannot be resized, there being no number to write back.
    #[prop(default = None, into)]
    width: Option<Signal<f64>>,
    #[prop(default = None, into)] height: Option<Signal<f64>>,
    /// Rotation about the node's own centre, in radians. Rendered, carried into every
    /// `CanvasGesture` and saved with the rest of the box — but there is no gesture that turns
    /// one yet, so today it is set from the caller's own data or not at all.
    ///
    /// A node with an angle draws **no grips**: `WorldBox::resized` takes its delta in world axes,
    /// which are not the box's axes once it is turned, so the grips would grow it in the wrong
    /// direction. Refusing to draw them is better than drawing eight that lie.
    #[prop(optional, into)]
    angle: Signal<f64>,
    /// Where a drag on the node itself would put it. Unset, the node does not move.
    #[prop(default = None, into)]
    on_move: Option<Callback<CanvasPoint>>,
    /// What a drag on one of the grips would make of it. Unset — or with no `width` and `height`
    /// to write back to — no grips are drawn.
    #[prop(default = None, into)]
    on_resize: Option<Callback<WorldBox>>,
    /// Width ÷ height, kept through a resize. `Some(1.0)` for a square note, and with one only
    /// the four corners are drawn: a side cannot honour a ratio without moving an edge the
    /// pointer is not on.
    #[prop(default = None, into)]
    aspect: Option<f64>,
    /// Run once when a move or a resize is over, with the box it began from and the box it ended
    /// at. Where a shared document writes its operation and an undo stack writes its entry — the
    /// live callbacks above are the fortieth intermediate frame, and this is the only one worth
    /// telling anybody else about.
    ///
    /// Silent for a press that changed nothing, so selecting a node is not an edit.
    #[prop(default = None, into)]
    on_gesture_end: Option<Callback<CanvasGesture>>,
    /// The smallest a resize may leave it, in world units.
    #[prop(default = 24.0)]
    min_size: f64,
    /// Whether this is the node the caller considers selected. Written as `data-selected`, and
    /// what the grips are drawn by — a board with eight grips on every node is unreadable.
    #[prop(optional, into)]
    selected: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    // Only a node that does something needs the canvas, so a plain one still renders outside it.
    let interactive = on_move.is_some() || on_resize.is_some();
    let ctx = interactive.then(use_canvas);

    let sized = width.is_some() && height.is_some();
    let resizable = on_resize.is_some() && sized;

    let current = move || {
        WorldBox::new(
            x.get_untracked(),
            y.get_untracked(),
            width.map_or(0.0, |width| width.get_untracked()),
            height.map_or(0.0, |height| height.get_untracked()),
        )
        .rotated(angle.get_untracked())
    };

    // The box as it was at the press. A resize is a box computed from the total travel rather
    // than one nudged by each move: clamped incrementally at `min_size`, the remainder is thrown
    // away and the node then lags the pointer by however far it was dragged past the limit.
    let from = StoredValue::new(WorldBox::default());
    let grip = StoredValue::new(None::<BoxHandle>);
    let dragging = RwSignal::new(false);

    let drag = use_drag(move |point| {
        let zoom = ctx.map_or(1.0, |ctx| ctx.viewport.get_untracked().zoom);
        if zoom == 0.0 {
            return;
        }
        let (dx, dy) = (point.dx / zoom, point.dy / zoom);
        let from = from.get_value();

        match grip.get_value() {
            Some(handle) => {
                if let Some(on_resize) = on_resize {
                    on_resize.run(from.resized(handle, dx, dy, min_size, aspect));
                }
            }
            None => {
                if let Some(on_move) = on_move {
                    on_move.run(CanvasPoint {
                        x: from.x + dx,
                        y: from.y + dy,
                    });
                }
            }
        }
    })
    .on_start(move |_| from.set_value(current()))
    // The release can land anywhere, including outside the node, so this is what ends the
    // gesture rather than a `pointerup` the node would never hear.
    .on_end(move |_| {
        let handle = grip.get_value();
        grip.set_value(None);
        dragging.set(false);

        let Some(on_gesture_end) = on_gesture_end else {
            return;
        };
        // Where it ended is read back out of the caller's own signals rather than recomputed, so
        // a caller that snapped the node to a grid, refused the move or clamped it to a lane
        // reports what it actually did rather than what it was asked to do.
        let (began, ended) = (from.get_value(), current());
        // `use_drag` ends a gesture on release whether or not the pointer ever moved, and a press
        // that changed nothing is a selection. Without this a board that commits what it is told
        // would put an operation on the wire every time somebody clicked something.
        if ended != began {
            on_gesture_end.run(CanvasGesture {
                from: began,
                to: ended,
                handle,
            });
        }
    });

    // Every gesture on a node starts the same way, and refuses for the same three reasons.
    let begin = move |e: &ev::PointerEvent, handle: Option<BoxHandle>| -> bool {
        if e.button() != 0 || ctx.is_some_and(|ctx| ctx.space_held.get_untracked()) {
            return false;
        }
        // The node's gesture is the node's: without this a surface with the hand tool on would
        // pan the board out from under a node being dragged across it.
        e.stop_propagation();
        grip.set_value(handle);
        dragging.set(true);
        drag.start(e);
        true
    };

    let handles: &[BoxHandle] = if aspect.is_some() {
        &BoxHandle::CORNERS
    } else {
        &BoxHandle::ALL
    };

    view! {
        <div
            data-slot="canvas-node"
            data-selected=move || selected.get().then_some("true")
            data-dragging=move || dragging.get().then_some("true")
            style=move || {
                let mut style = format!(
                    "position: absolute; left: {}px; top: {}px;",
                    x.get(),
                    y.get(),
                );
                if let Some(width) = width {
                    style.push_str(&format!(" width: {}px;", width.get()));
                }
                if let Some(height) = height {
                    style.push_str(&format!(" height: {}px;", height.get()));
                }
                // Written only when there is one, so the overwhelmingly common upright node is
                // the same element it always was rather than one the browser has to composite.
                let angle = angle.get();
                if angle != 0.0 {
                    style.push_str(&format!(" transform: rotate({angle}rad);"));
                }
                style
            }
            on:pointerdown=move |e: ev::PointerEvent| {
                if on_move.is_some() {
                    begin(&e, None);
                }
            }
            class=class
        >
            {children()}
            {move || {
                (resizable && angle.get() == 0.0)
                    .then(|| {
                        handles
                            .iter()
                            .map(|handle| {
                                let handle = *handle;
                                let (fraction_x, fraction_y) = handle.fractions();
                                view! {
                                    <div
                                        data-slot="canvas-node-handle"
                                        data-handle=handle.as_str()
                                        data-corner=handle.is_corner().then_some("")
                                        style=move || {
                                            let zoom = ctx
                                                .map_or(1.0, |ctx| ctx.viewport.get().zoom)
                                                .max(f64::MIN_POSITIVE);
                                            let size = HANDLE_SIZE / zoom;
                                            format!(
                                                "position: absolute; left: {}%; top: {}%; width: {size}px; height: {size}px; transform: translate(-50%, -50%);",
                                                fraction_x * 100.0,
                                                fraction_y * 100.0,
                                            )
                                        }
                                        on:pointerdown=move |e: ev::PointerEvent| {
                                            begin(&e, Some(handle));
                                        }
                                    />
                                }
                            })
                            .collect_view()
                    })
            }}
        </div>
    }
}

/// A node's label, which becomes an input when you double-click it.
///
/// Editing text on a canvas is not the canvas' business — a node's children are the caller's —
/// but the seam between editing and the gestures the node already has is. Two rules, and they
/// point opposite ways on purpose: the **text** lets a press through, so a sticky can still be
/// dragged by its words; the **input** stops one, because that press belongs to the caret.
///
/// A **textarea** rather than an input, because the text it replaces wraps and an input does not:
/// on a sticky note the words would reflow onto one line the moment you began editing them, and
/// jump back when you stopped. It is sized by its content, so it is the same shape as the words
/// it stands in for.
///
/// The text is rendered with its line breaks kept — `white-space: pre-wrap` in the styled layer —
/// because a key that puts a break in has to be a key whose break is still there afterwards. A
/// span collapses one to a space by default, which is a multi-line note that reads as one line the
/// moment you stop editing it.
///
/// The value stays the caller's. **Enter breaks the line** — the key that makes a new line in
/// every other multi-line field should not be the one that closes this one. Everything else ends
/// the edit and keeps the text: **Escape**, **⌘/Ctrl+Enter**, and **clicking away**. There is no
/// cancel on purpose: the words under the caret are the note, so a key that silently threw them
/// away would be a key nobody presses twice.
#[component]
pub fn CanvasNodeLabelRoot(
    #[prop(into)] value: Signal<String>,
    #[prop(default = None, into)] on_change: Option<Callback<String>>,
    /// Whether the input is showing. A signal rather than internal state so a caller can start an
    /// edit from somewhere else — a menu item, a keystroke on the board.
    #[prop(optional, into)]
    editing: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let input_ref = AnyNodeRef::new();
    // Whether a key has already finished this edit.
    //
    // Chrome fires `blur` at an element it takes out of the document, so a key that ends the edit
    // is followed by one — and the blur would then report the text a second time. Harmless while
    // the caller applies what it is told and the second report compares equal, and not harmless
    // at all for a caller that rejects an edit, which would see the same one arrive twice.
    // Cleared when an edit begins rather than by the blur, so a browser that does not send that
    // blur cannot leave the flag set for the next edit to trip over.
    let settled = StoredValue::new(false);

    // Focus after the input exists, and select what is in it: a rename that begins with the old
    // text selected is a rename you can type straight over.
    Effect::new(move |_| {
        if editing.get()
            && let Some(input) = input_ref.get()
            && let Ok(input) = input.dyn_into::<HtmlTextAreaElement>()
        {
            settled.set_value(false);
            let _ = input.focus();
            input.select();
        }
    });

    // Every way out of an edit is the same way out: keep the text and stop editing. There is no
    // cancel, because on a board there is nothing to cancel back to that the reader can see — the
    // words under the caret are the note, and a key that silently threw them away would be a key
    // nobody presses twice.
    let finish = move |input: &HtmlTextAreaElement| {
        settled.set_value(true);
        editing.set(false);
        let next = input.value();
        if let Some(on_change) = on_change
            && next != value.get_untracked()
        {
            on_change.run(next);
        }
    };

    view! {
        <Show
            when=move || editing.get()
            fallback=move || {
                view! {
                    <span
                        data-slot="canvas-node-label"
                        class=class
                        // Not a click, and not a press: a node is dragged with the same button,
                        // so a rename that fired on either would fire on every drag that began
                        // on the words. The press goes through to the node on purpose.
                        on:dblclick=move |e: ev::MouseEvent| {
                            e.stop_propagation();
                            editing.set(true);
                        }
                    >
                        {move || value.get()}
                    </span>
                }
            }
        >
            <textarea
                node_ref=input_ref
                data-slot="canvas-node-label"
                data-editing="true"
                // One line to begin with, and it grows from there — the styled layer sizes it by
                // its content, and a textarea's own default of two rows would leave a blank line
                // under every one-line note.
                rows="1"
                class=class
                prop:value=move || value.get_untracked()
                // The press belongs to the caret, not to the node underneath it.
                on:pointerdown=move |e: ev::PointerEvent| e.stop_propagation()
                on:dblclick=move |e: ev::MouseEvent| e.stop_propagation()
                // Clicking away is the commonest way to be done with a note, and the one nobody
                // has to be taught.
                on:blur=move |e: ev::FocusEvent| {
                    if !settled.get_value() {
                        finish(&event_target::<HtmlTextAreaElement>(&e));
                    }
                }
                on:keydown=move |e: ev::KeyboardEvent| {
                    match e.key().as_str() {
                        // A *plain* Enter is not here on purpose: it falls through to the
                        // arm below, which neither finishes the edit nor prevents the default,
                        // so the line break goes in the way it does in any other textarea.
                        //
                        // Both of these are stopped as well as prevented: the key that ended a
                        // rename should not go on to close the dialog the board sits in.
                        "Enter" if e.meta_key() || e.ctrl_key() => {
                            e.prevent_default();
                            e.stop_propagation();
                            finish(&event_target::<HtmlTextAreaElement>(&e));
                        }
                        "Escape" => {
                            e.prevent_default();
                            e.stop_propagation();
                            finish(&event_target::<HtmlTextAreaElement>(&e));
                        }
                        // Every other key is the textarea's, Enter among them. Stopped rather
                        // than left alone: they reach the surface otherwise, where ⌫ deletes the
                        // very node being renamed. `stop_propagation` does not touch the default
                        // action, so the typing still happens.
                        _ => e.stop_propagation(),
                    }
                }
            />
        </Show>
    }
}
