//! A value chosen by dragging along a track.
//!
//! The value is a `Vec<f64>` and a range slider is the two-element case, so the geometry, the
//! keyboard contract and the ARIA are written once. Thumbs clamp to their neighbours.
//!
//! Dragging is tracked on the window: a pointer that leaves the element mid-drag still belongs to
//! the drag. Positions are geometry, so this layer writes the inline `left`/`width`.

use crate::utils::types::Orientation;
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement};

#[derive(Copy, Clone)]
pub struct SliderContext {
    pub values: RwSignal<Vec<f64>>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: Orientation,
    pub disabled: Signal<bool>,
    /// The thumb currently being dragged, so the styled layer can mark it.
    pub active_thumb: RwSignal<Option<usize>>,
    pub track_ref: AnyNodeRef,
}

/// How many decimals `step` is written to, so snapping does not turn 0.3 into 0.30000000000000004.
fn decimals_of(step: f64) -> i32 {
    match format!("{step}").split_once('.') {
        Some((_, frac)) => frac.len() as i32,
        None => 0,
    }
}

impl SliderContext {
    /// Where a value sits along the track, 0.0 at `min` and 1.0 at `max`.
    pub fn percent(&self, value: f64) -> f64 {
        if self.max <= self.min {
            return 0.0;
        }
        ((value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    fn snap(&self, value: f64) -> f64 {
        if self.step <= 0.0 {
            return value.clamp(self.min, self.max);
        }

        let steps = ((value - self.min) / self.step).round();
        let snapped = (self.min + steps * self.step).clamp(self.min, self.max);
        let factor = 10f64.powi(decimals_of(self.step));

        (snapped * factor).round() / factor
    }

    /// The bounds a thumb may move between: the track ends, or its neighbours.
    pub fn bounds(&self, index: usize) -> (f64, f64) {
        self.values.with(|values| {
            let lower = if index == 0 {
                self.min
            } else {
                values[index - 1]
            };
            let upper = if index + 1 >= values.len() {
                self.max
            } else {
                values[index + 1]
            };
            (lower, upper)
        })
    }

    pub fn value_at(&self, index: usize) -> f64 {
        self.values
            .with(|values| values.get(index).copied().unwrap_or(self.min))
    }

    pub fn set_thumb(&self, index: usize, value: f64) {
        let (lower, upper) = self.bounds(index);
        let next = self.snap(value).clamp(lower, upper);

        self.values.update(|values| {
            if let Some(slot) = values.get_mut(index)
                && *slot != next
            {
                *slot = next;
            }
        });
    }

    /// Nudge a thumb by whole steps, for the keyboard.
    pub fn step_thumb(&self, index: usize, steps: f64) {
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        self.set_thumb(index, self.value_at(index) + steps * step);
    }

    /// The value under a pointer, from the track's own box.
    pub fn value_from_pointer(&self, client_x: f64, client_y: f64) -> Option<f64> {
        let track = self.track_ref.get_untracked()?;
        let rect = track.dyn_ref::<Element>()?.get_bounding_client_rect();

        let fraction = match self.orientation {
            Orientation::Horizontal => {
                if rect.width() <= 0.0 {
                    return None;
                }
                (client_x - rect.left()) / rect.width()
            }
            // A vertical slider runs bottom-to-top.
            Orientation::Vertical => {
                if rect.height() <= 0.0 {
                    return None;
                }
                1.0 - (client_y - rect.top()) / rect.height()
            }
        };

        Some(self.min + fraction.clamp(0.0, 1.0) * (self.max - self.min))
    }

    /// The thumb a press on the track picks up. Ties go to the lower thumb, except at the top of
    /// the range, where it has nowhere to go.
    pub fn closest_thumb(&self, value: f64) -> usize {
        self.values.with(|values| {
            let mut best = 0;
            let mut best_distance = f64::MAX;

            for (index, thumb) in values.iter().enumerate() {
                let distance = (thumb - value).abs();
                if distance < best_distance {
                    best_distance = distance;
                    best = index;
                }
            }

            best
        })
    }
}

pub fn use_slider() -> SliderContext {
    expect_context::<SliderContext>()
}

#[component]
pub fn SliderRoot(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    /// Omit for an uncontrolled slider. The length decides how many thumbs there are.
    #[prop(default = None, into)]
    value: Option<RwSignal<Vec<f64>>>,
    #[prop(optional)] default_value: Option<Vec<f64>>,
    children: Children,
) -> impl IntoView {
    let values = value.unwrap_or_else(|| RwSignal::new(default_value.unwrap_or_else(|| vec![min])));

    let context = SliderContext {
        values,
        min,
        max,
        step,
        orientation,
        disabled,
        active_thumb: RwSignal::new(None),
        track_ref: AnyNodeRef::new(),
    };

    // Installed per gesture and removed on release, or every slider on the page would react to
    // every pointer move for the rest of the session.
    let move_handle = StoredValue::new_local(None::<WindowListenerHandle>);
    let up_handle = StoredValue::new_local(None::<WindowListenerHandle>);

    // `take` rather than read-then-clear: a listener handle is not `Clone`.
    let stop_drag = move || {
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
        context.active_thumb.set(None);
    };

    let start_drag = move |e: ev::PointerEvent| {
        if disabled.get_untracked() {
            return;
        }

        let Some(value) = context.value_from_pointer(e.client_x() as f64, e.client_y() as f64)
        else {
            return;
        };

        // The nearest thumb moves to the press and is picked up, so a click and a drag are one
        // gesture.
        let index = context.closest_thumb(value);
        context.active_thumb.set(Some(index));
        context.set_thumb(index, value);
        focus_thumb(context.track_ref, index);

        stop_drag();

        move_handle.set_value(Some(window_event_listener(
            ev::pointermove,
            move |e: ev::PointerEvent| {
                if let Some(value) =
                    context.value_from_pointer(e.client_x() as f64, e.client_y() as f64)
                {
                    context.set_thumb(index, value);
                }
            },
        )));

        up_handle.set_value(Some(window_event_listener(ev::pointerup, move |_| {
            stop_drag();
        })));
    };

    on_cleanup(stop_drag);

    let orientation_str = orientation.as_str();

    view! {
        <Provider value=context>
            <div
                data-slot="slider"
                data-orientation=orientation_str
                data-disabled=move || disabled.get().then_some("true")
                on:pointerdown=start_drag
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Moves focus onto the thumb being dragged, so a release leaves the keyboard on it.
fn focus_thumb(track_ref: AnyNodeRef, index: usize) {
    if let Some(track) = track_ref.get_untracked()
        && let Some(root) = track
            .dyn_ref::<Element>()
            .and_then(|e| e.closest("[data-slot=slider]").ok().flatten())
        && let Ok(thumbs) = root.query_selector_all("[data-slot=slider-thumb]")
        && let Some(thumb) = thumbs.item(index as u32)
        && let Ok(thumb) = thumb.dyn_into::<HtmlElement>()
    {
        let _ = thumb.focus();
    }
}

#[component]
pub fn SliderTrackRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_slider();

    view! {
        <div
            node_ref=ctx.track_ref
            data-slot="slider-track"
            data-orientation=ctx.orientation.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

/// The filled stretch of track: from the start to the only thumb, or between the outer two.
#[component]
pub fn SliderRangeRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_slider();

    let span = move || {
        let (from, to) = ctx.values.with(|values| {
            let lowest = values.iter().copied().fold(f64::MAX, f64::min);
            let highest = values.iter().copied().fold(f64::MIN, f64::max);
            if values.len() > 1 {
                (lowest, highest)
            } else {
                (ctx.min, highest)
            }
        });

        let start = ctx.percent(from) * 100.0;
        let length = (ctx.percent(to) * 100.0 - start).max(0.0);

        match ctx.orientation {
            Orientation::Horizontal => format!("left: {start}%; width: {length}%;"),
            Orientation::Vertical => format!("bottom: {start}%; height: {length}%;"),
        }
    };

    view! {
        <div
            data-slot="slider-range"
            data-orientation=ctx.orientation.as_str()
            style=span
            class=class
        ></div>
    }
}

#[component]
pub fn SliderThumbRoot(
    /// Which value this thumb owns.
    #[prop(default = 0)]
    index: usize,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_slider();

    let position = move || {
        let percent = ctx.percent(ctx.value_at(index)) * 100.0;
        match ctx.orientation {
            // Centres the thumb on its value rather than hanging it off the track's ends.
            Orientation::Horizontal => format!("left: {percent}%; transform: translateX(-50%);"),
            Orientation::Vertical => format!("bottom: {percent}%; transform: translateY(50%);"),
        }
    };

    let on_keydown = move |e: ev::KeyboardEvent| {
        if ctx.disabled.get_untracked() {
            return;
        }

        let handled = match e.key().as_str() {
            "ArrowRight" | "ArrowUp" => {
                ctx.step_thumb(index, 1.0);
                true
            }
            "ArrowLeft" | "ArrowDown" => {
                ctx.step_thumb(index, -1.0);
                true
            }
            "PageUp" => {
                ctx.step_thumb(index, 10.0);
                true
            }
            "PageDown" => {
                ctx.step_thumb(index, -10.0);
                true
            }
            "Home" => {
                ctx.set_thumb(index, ctx.min);
                true
            }
            "End" => {
                ctx.set_thumb(index, ctx.max);
                true
            }
            _ => false,
        };

        if handled {
            e.prevent_default();
        }
    };

    view! {
        <span
            role="slider"
            data-slot="slider-thumb"
            data-orientation=ctx.orientation.as_str()
            data-disabled=move || ctx.disabled.get().then_some("true")
            data-state=move || if ctx.active_thumb.get() == Some(index) { "dragging" } else { "idle" }
            tabindex=move || if ctx.disabled.get() { "-1" } else { "0" }
            aria-orientation=ctx.orientation.as_str()
            // The thumb's own bounds, not the track's: it cannot pass its neighbour.
            aria-valuemin=move || ctx.bounds(index).0
            aria-valuemax=move || ctx.bounds(index).1
            aria-valuenow=move || ctx.value_at(index)
            aria-disabled=move || ctx.disabled.get().then_some("true")
            on:keydown=on_keydown
            style=position
            class=class
        ></span>
    }
}
