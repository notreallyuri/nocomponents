//! A panel the reader can pick up and put somewhere else.
//!
//! Fixed to the viewport rather than placed in the flow, because that is what "move it out of the
//! way" means: it should stay where it was put while the page scrolls under it. Everything else
//! here follows from that — the position is client coordinates, and the clamp is against the
//! window rather than against a parent.
//!
//! **Dragged by a handle, not by the whole panel.** A panel with buttons in it that also moves
//! when you press a button is a panel you cannot use; [`FloatingMenuHandleRoot`] is the part that
//! takes the press. A caller who *does* want the whole thing to move puts the handle around
//! everything, which is a decision they can make and this cannot.
//!
//! The position is the caller's if they pass one. Uncontrolled it is still a signal, so a caller
//! who only wants to *read* where the reader put it can pass one in and never write to it.

use crate::primitives::drag::use_drag;
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// How much of the panel must stay on screen. Enough to grab it back by.
const KEEP_VISIBLE: f64 = 32.0;

#[derive(Copy, Clone)]
pub struct FloatingMenuContext {
    /// Top-left, in client coordinates.
    pub position: RwSignal<(f64, f64)>,
    /// Whether the panel is being moved right now.
    pub dragging: RwSignal<bool>,
    /// The panel, so the handle can measure it to keep it on screen.
    pub panel_ref: AnyNodeRef,
}

pub fn use_floating_menu() -> FloatingMenuContext {
    expect_context::<FloatingMenuContext>()
}

/// The window, as the clamp measures against it.
fn viewport() -> (f64, f64) {
    let width = window().inner_width().ok().and_then(|v| v.as_f64());
    let height = window().inner_height().ok().and_then(|v| v.as_f64());
    (width.unwrap_or(0.0), height.unwrap_or(0.0))
}

/// Keeps `position` far enough inside `view` that the panel can still be reached.
///
/// Against the *window* rather than the document: a panel fixed to the viewport is put where the
/// reader can see it, and a page that scrolls afterwards must not take it away.
///
/// Only the panel's width is wanted. It may hang off either side with a grabbable strip still
/// showing, and off the bottom the same way — but never off the top, because a handle dragged
/// above the top edge has nothing above it to reach for and cannot be brought back.
fn clamp(position: (f64, f64), width: f64, view: (f64, f64)) -> (f64, f64) {
    let (view_width, view_height) = view;
    if view_width <= 0.0 || view_height <= 0.0 {
        return position;
    }

    let (x, y) = position;
    (
        x.clamp(KEEP_VISIBLE - width, view_width - KEEP_VISIBLE),
        y.clamp(0.0, view_height - KEEP_VISIBLE),
    )
}

#[component]
pub fn FloatingMenuRoot(
    /// Where it sits, in client coordinates. Pass one to drive it from outside — or to read where
    /// the reader left it.
    #[prop(default = None)]
    position: Option<RwSignal<(f64, f64)>>,
    /// Where it starts when the position is not passed in.
    #[prop(default = (24.0, 24.0))]
    default_position: (f64, f64),
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    children: Children,
) -> impl IntoView {
    let panel_ref = AnyNodeRef::new();
    let context = FloatingMenuContext {
        position: position.unwrap_or_else(|| RwSignal::new(default_position)),
        dragging: RwSignal::new(false),
        panel_ref,
    };

    // A window that narrows can leave the panel off the edge, and nothing else would put it back.
    let resize = window_event_listener(ev::resize, move |_| {
        let Some(panel) = context.panel_ref.get_untracked() else {
            return;
        };
        let rect = panel.get_bounding_client_rect();
        let put = clamp(context.position.get_untracked(), rect.width(), viewport());
        if put != context.position.get_untracked() {
            context.position.set(put);
        }
    });
    on_cleanup(move || resize.remove());

    view! {
        <div
            node_ref=panel_ref
            data-slot="floating-menu"
            data-dragging=move || context.dragging.get().then_some("true")
            style=move || {
                let (x, y) = context.position.get();
                format!("position: fixed; left: {x}px; top: {y}px; {}", style.get())
            }
            class=class
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
}

/// The part of the panel a press moves it by.
#[component]
pub fn FloatingMenuHandleRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_floating_menu();
    // Where the panel was when the press landed. The gesture reports how far the pointer has come
    // since then, so the two together are where it should be now — which is why a drag cannot
    // drift no matter how many moves it takes.
    let origin = StoredValue::new((0.0f64, 0.0f64));

    let drag = use_drag(move |point| {
        let (ox, oy) = origin.get_value();
        let put = (ox + point.dx, oy + point.dy);

        let width = ctx
            .panel_ref
            .get_untracked()
            .map(|panel| panel.get_bounding_client_rect().width())
            .unwrap_or(0.0);

        ctx.position.set(clamp(put, width, viewport()));
    })
    .on_start(move |_| {
        origin.set_value(ctx.position.get_untracked());
        ctx.dragging.set(true);
    })
    .on_end(move |_| ctx.dragging.set(false));

    view! {
        <div
            data-slot="floating-menu-handle"
            data-dragging=move || ctx.dragging.get().then_some("true")
            on:pointerdown=move |e: ev::PointerEvent| {
                // The primary button, and never from a control the handle happens to wrap.
                if disabled.get_untracked() || e.button() != 0 {
                    return;
                }
                e.prevent_default();
                drag.start(&e);
            }
            class=class
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{KEEP_VISIBLE, clamp};

    const VIEW: (f64, f64) = (1000.0, 800.0);

    #[test]
    fn a_position_well_inside_is_left_alone() {
        assert_eq!(clamp((120.0, 90.0), 200.0, VIEW), (120.0, 90.0));
    }

    #[test]
    fn it_may_hang_off_a_side_with_a_strip_still_showing() {
        // Far off the left: the panel keeps `KEEP_VISIBLE` of its right edge on screen.
        assert_eq!(clamp((-900.0, 40.0), 200.0, VIEW).0, KEEP_VISIBLE - 200.0);
        // Far off the right: its left edge stops that far in from the window's.
        assert_eq!(clamp((5000.0, 40.0), 200.0, VIEW).0, 1000.0 - KEEP_VISIBLE);
    }

    #[test]
    fn it_never_goes_above_the_top() {
        // Not symmetric with the sides on purpose: there is nothing above the top edge to grab.
        assert_eq!(clamp((100.0, -400.0), 200.0, VIEW).1, 0.0);
    }

    #[test]
    fn it_may_hang_off_the_bottom() {
        assert_eq!(clamp((100.0, 5000.0), 200.0, VIEW).1, 800.0 - KEEP_VISIBLE);
    }

    #[test]
    fn a_window_of_no_size_clamps_nothing() {
        // Before the first layout there is nothing to clamp against, and guessing would move a
        // panel the caller had placed.
        assert_eq!(clamp((10.0, 20.0), 100.0, (0.0, 0.0)), (10.0, 20.0));
    }
}
