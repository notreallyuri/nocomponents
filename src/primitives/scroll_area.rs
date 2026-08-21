//! A scrolling box with scrollbars this library draws itself.
//!
//! The box still scrolls natively; only the painting of the scrollbar is replaced, because a
//! native one cannot be styled consistently across platforms. The viewport hides its own bars and
//! this measures it, publishing thumb geometry in pixels.
//!
//! Measuring is driven by a `ResizeObserver` on the viewport *and* its content as well as by the
//! scroll event: content that grows after mount changes the thumb without any scroll firing.

use crate::{
    primitives::drag::{DragPoint, use_drag},
    utils::types::Orientation,
};
use leptos::{
    context::Provider,
    ev,
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement, ResizeObserver};

/// A thumb never shrinks below this: a two-pixel thumb is a target nobody can hit.
const MIN_THUMB: f64 = 20.0;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct ScrollMetrics {
    /// Whether the content overflows on this axis; the styled layer hides the bar when it does
    /// not.
    pub overflowing: bool,
    /// Thumb length along the track, in pixels.
    pub size: f64,
    /// Thumb distance from the start of the track, in pixels.
    pub offset: f64,
}

#[derive(Copy, Clone)]
pub struct ScrollAreaContext {
    pub viewport_ref: AnyNodeRef,
    pub vertical: RwSignal<ScrollMetrics>,
    pub horizontal: RwSignal<ScrollMetrics>,
}

impl ScrollAreaContext {
    pub fn metrics(&self, orientation: Orientation) -> ScrollMetrics {
        match orientation {
            Orientation::Vertical => self.vertical.get(),
            Orientation::Horizontal => self.horizontal.get(),
        }
    }

    pub fn viewport(&self) -> Option<HtmlElement> {
        self.viewport_ref
            .get_untracked()
            .and_then(|node| node.dyn_into::<HtmlElement>().ok())
    }

    /// Re-measure both axes from the live element.
    pub fn measure(&self) {
        let Some(viewport) = self.viewport() else {
            return;
        };

        let next_vertical = axis_metrics(
            viewport.client_height() as f64,
            viewport.scroll_height() as f64,
            viewport.scroll_top() as f64,
        );
        let next_horizontal = axis_metrics(
            viewport.client_width() as f64,
            viewport.scroll_width() as f64,
            viewport.scroll_left() as f64,
        );

        // Most scrolls do not move the thumb, and those must not wake every subscriber.
        if self.vertical.get_untracked() != next_vertical {
            self.vertical.set(next_vertical);
        }
        if self.horizontal.get_untracked() != next_horizontal {
            self.horizontal.set(next_horizontal);
        }
    }
}

fn axis_metrics(client: f64, scroll: f64, offset: f64) -> ScrollMetrics {
    // A pixel of slack for sub-pixel layout, which reports content that fits as overflowing.
    if client <= 0.0 || scroll <= client + 1.0 {
        return ScrollMetrics::default();
    }

    let size = (client * (client / scroll)).max(MIN_THUMB).min(client);
    let max_scroll = scroll - client;
    let max_offset = client - size;
    let progress = if max_scroll > 0.0 {
        (offset / max_scroll).clamp(0.0, 1.0)
    } else {
        0.0
    };

    ScrollMetrics {
        overflowing: true,
        size,
        offset: progress * max_offset,
    }
}

pub fn use_scroll_area() -> ScrollAreaContext {
    expect_context::<ScrollAreaContext>()
}

#[component]
pub fn ScrollAreaRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let context = ScrollAreaContext {
        viewport_ref: AnyNodeRef::new(),
        vertical: RwSignal::new(ScrollMetrics::default()),
        horizontal: RwSignal::new(ScrollMetrics::default()),
    };

    view! {
        <Provider value=context>
            <div data-slot="scroll-area" class=class>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn ScrollAreaViewportRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_scroll_area();
    let content_ref = AnyNodeRef::new();
    let observer = StoredValue::new_local(None::<ResizeObserver>);

    Effect::new(move |_| {
        let (Some(viewport), Some(content)) = (
            ctx.viewport_ref
                .get()
                .and_then(|n| n.dyn_into::<Element>().ok()),
            content_ref.get().and_then(|n| n.dyn_into::<Element>().ok()),
        ) else {
            return;
        };

        ctx.measure();

        if observer.with_value(|o| o.is_some()) {
            return;
        }

        // The viewport for the box resizing, the content for it growing underneath.
        let callback = Closure::<dyn Fn()>::new(move || ctx.measure());
        if let Ok(resize_observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            resize_observer.observe(&viewport);
            resize_observer.observe(&content);
            observer.set_value(Some(resize_observer));
        }
        // The observer holds the closure; both go when it is disconnected below.
        callback.forget();
    });

    on_cleanup(move || {
        observer.update_value(|slot| {
            if let Some(observer) = slot.take() {
                observer.disconnect();
            }
        });
    });

    view! {
        <div
            node_ref=ctx.viewport_ref
            data-slot="scroll-area-viewport"
            // Functional: the box scrolls and its native bars go. WebKit needs a pseudo-element
            // rule an inline style cannot express, which the styled layer carries.
            style="overflow: auto; scrollbar-width: none; -ms-overflow-style: none;"
            on:scroll=move |_| ctx.measure()
            class=class
        >
            <div node_ref=content_ref data-slot="scroll-area-content">
                {children()}
            </div>
        </div>
    }
}

/// The track. A press jumps the thumb to the pointer rather than paging.
#[component]
pub fn ScrollAreaScrollbarRoot(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_scroll_area();
    let track_ref = AnyNodeRef::new();

    let jump_to = move |e: ev::PointerEvent| {
        // A press on the thumb is the start of a drag, which the thumb handles.
        if let Some(target) = e.target()
            && let Ok(target) = target.dyn_into::<Element>()
            && matches!(target.closest("[data-slot=scroll-area-thumb]"), Ok(Some(_)))
        {
            return;
        }

        let (Some(viewport), Some(track)) = (
            ctx.viewport(),
            track_ref
                .get_untracked()
                .and_then(|n| n.dyn_into::<Element>().ok()),
        ) else {
            return;
        };

        let metrics = ctx.metrics(orientation);
        if !metrics.overflowing {
            return;
        }

        let rect = track.get_bounding_client_rect();

        let (pointer, track_length, client, scroll) = match orientation {
            Orientation::Vertical => (
                e.client_y() as f64 - rect.top(),
                rect.height(),
                viewport.client_height() as f64,
                viewport.scroll_height() as f64,
            ),
            Orientation::Horizontal => (
                e.client_x() as f64 - rect.left(),
                rect.width(),
                viewport.client_width() as f64,
                viewport.scroll_width() as f64,
            ),
        };

        let max_offset = track_length - metrics.size;
        if max_offset <= 0.0 {
            return;
        }

        // Centre the thumb on the press.
        let progress = ((pointer - metrics.size / 2.0) / max_offset).clamp(0.0, 1.0);
        let target = progress * (scroll - client);

        match orientation {
            Orientation::Vertical => viewport.set_scroll_top(target as i32),
            Orientation::Horizontal => viewport.set_scroll_left(target as i32),
        }
    };

    view! {
        <div
            node_ref=track_ref
            data-slot="scroll-area-scrollbar"
            data-orientation=orientation.as_str()
            data-state=move || {
                if ctx.metrics(orientation).overflowing { "visible" } else { "hidden" }
            }
            on:pointerdown=jump_to
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ScrollAreaThumbRoot(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_scroll_area();

    // Where the content was when the thumb was taken hold of. The thumb tracks the pointer
    // absolutely from there, so a drag that overshoots and comes back lands where it started.
    let start_scroll = StoredValue::new(0.0);

    let drag = use_drag(move |point: DragPoint| {
        let Some(viewport) = ctx.viewport() else {
            return;
        };
        let metrics = ctx.metrics(orientation);

        let (travelled, client, scroll) = match orientation {
            Orientation::Vertical => (
                point.dy,
                viewport.client_height() as f64,
                viewport.scroll_height() as f64,
            ),
            Orientation::Horizontal => (
                point.dx,
                viewport.client_width() as f64,
                viewport.scroll_width() as f64,
            ),
        };

        let max_offset = client - metrics.size;
        if max_offset <= 0.0 {
            return;
        }

        // The thumb travels `max_offset` while the content travels `scroll - client`.
        let target =
            (start_scroll.get_value() + travelled * ((scroll - client) / max_offset)).max(0.0);

        match orientation {
            Orientation::Vertical => viewport.set_scroll_top(target as i32),
            Orientation::Horizontal => viewport.set_scroll_left(target as i32),
        }
    });

    let start_drag = move |e: ev::PointerEvent| {
        let Some(viewport) = ctx.viewport() else {
            return;
        };

        // The press must not reach the track, which would jump before dragging.
        e.stop_propagation();

        start_scroll.set_value(match orientation {
            Orientation::Vertical => viewport.scroll_top() as f64,
            Orientation::Horizontal => viewport.scroll_left() as f64,
        });

        drag.start(&e);
    };

    let position = move || {
        let metrics = ctx.metrics(orientation);
        match orientation {
            Orientation::Vertical => format!(
                "height: {}px; transform: translateY({}px);",
                metrics.size, metrics.offset
            ),
            Orientation::Horizontal => format!(
                "width: {}px; transform: translateX({}px);",
                metrics.size, metrics.offset
            ),
        }
    };

    view! {
        <div
            data-slot="scroll-area-thumb"
            data-orientation=orientation.as_str()
            data-state=move || if drag.active.get() { "dragging" } else { "idle" }
            on:pointerdown=start_drag
            style=position
            class=class
        ></div>
    }
}
