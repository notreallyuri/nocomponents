//! A carousel: a strip of slides you move through one at a time.
//!
//! Built on the browser's own scroll snapping rather than on a transform. The track is a real
//! scroll container whose children snap, which means a trackpad, a touch drag and a shift-wheel
//! all work without a line of code, and moving to a slide is a scroll rather than an animation to
//! keep in step with. `scroll-behavior: smooth` on the track is what makes that scroll animate,
//! so the styled layer owns the feel and this file owns only the arithmetic.
//!
//! Which slide is current is *derived*, never assumed: after any scroll — ours or the user's —
//! the nearest slide to the start of the track wins. Nothing can drift out of step with what is
//! actually on screen.

use crate::utils::types::Orientation;
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

/// The attribute that marks a slide, so the track can find them in document order.
pub const CAROUSEL_ITEM: &str = "data-carousel-item";

#[derive(Copy, Clone)]
pub struct CarouselContext {
    /// The slide at the start of the track.
    pub index: RwSignal<usize>,
    /// How many slides there are, kept by the slides themselves.
    pub count: RwSignal<usize>,
    pub orientation: Orientation,
    /// Whether moving past either end comes round the other side.
    pub wrap: bool,
    /// The scroll container.
    pub track_ref: AnyNodeRef,
}

impl CarouselContext {
    fn items(&self) -> Vec<HtmlElement> {
        let Some(track) = self.track_ref.get_untracked() else {
            return Vec::new();
        };
        let Ok(found) = track.query_selector_all(&format!("[{CAROUSEL_ITEM}]")) else {
            return Vec::new();
        };

        (0..found.length())
            .filter_map(|i| found.item(i))
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .collect()
    }

    /// Where a slide sits relative to the start of the track, in pixels of scroll.
    fn offset_of(&self, item: &HtmlElement) -> Option<f64> {
        let track = self.track_ref.get_untracked()?;
        let track_rect = track.get_bounding_client_rect();
        let item_rect = item.get_bounding_client_rect();

        Some(match self.orientation {
            Orientation::Horizontal => {
                item_rect.left() - track_rect.left() + track.scroll_left() as f64
            }
            Orientation::Vertical => item_rect.top() - track_rect.top() + track.scroll_top() as f64,
        })
    }

    /// Scrolls slide `index` to the start of the track. The scroll itself is animated by CSS.
    pub fn scroll_to(&self, index: usize) {
        let items = self.items();
        let Some(item) = items.get(index) else {
            return;
        };
        let (Some(track), Some(offset)) = (self.track_ref.get_untracked(), self.offset_of(item))
        else {
            return;
        };

        match self.orientation {
            Orientation::Horizontal => track.set_scroll_left(offset.round() as i32),
            Orientation::Vertical => track.set_scroll_top(offset.round() as i32),
        }

        // Set straight away rather than waiting for the scroll to report back, so a button
        // disables itself on the press instead of a frame later.
        self.index.set(index);
    }

    /// Reads the current slide back out of the scroll position: the one nearest the start.
    pub fn sync_index(&self) {
        let items = self.items();
        if items.is_empty() {
            return;
        }
        let Some(track) = self.track_ref.get_untracked() else {
            return;
        };

        let scrolled = match self.orientation {
            Orientation::Horizontal => track.scroll_left() as f64,
            Orientation::Vertical => track.scroll_top() as f64,
        };

        let nearest = items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| self.offset_of(item).map(|offset| (i, offset)))
            .min_by(|(_, a), (_, b)| {
                (a - scrolled)
                    .abs()
                    .partial_cmp(&(b - scrolled).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i);

        if let Some(nearest) = nearest
            && self.index.get_untracked() != nearest
        {
            self.index.set(nearest);
        }
    }

    pub fn can_scroll_prev(&self) -> bool {
        self.wrap || self.index.get() > 0
    }

    pub fn can_scroll_next(&self) -> bool {
        self.wrap || self.index.get() + 1 < self.count.get()
    }

    pub fn prev(&self) {
        let count = self.count.get_untracked();
        if count == 0 {
            return;
        }
        let index = self.index.get_untracked();

        match (index, self.wrap) {
            (0, true) => self.scroll_to(count - 1),
            (0, false) => {}
            _ => self.scroll_to(index - 1),
        }
    }

    pub fn next(&self) {
        let count = self.count.get_untracked();
        if count == 0 {
            return;
        }
        let index = self.index.get_untracked();

        if index + 1 < count {
            self.scroll_to(index + 1);
        } else if self.wrap {
            self.scroll_to(0);
        }
    }
}

pub fn use_carousel() -> CarouselContext {
    expect_context::<CarouselContext>()
}

#[component]
pub fn CarouselRoot(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    /// Whether moving past either end comes round the other side.
    #[prop(default = false)]
    wrap: bool,
    /// The current slide, for a caller that wants to read it or jump elsewhere.
    #[prop(optional, into)]
    index: RwSignal<usize>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = CarouselContext {
        index,
        count: RwSignal::new(0),
        orientation,
        wrap,
        track_ref: AnyNodeRef::new(),
    };

    provide_context(ctx);

    view! {
        <Provider value=ctx>
            <section
                data-slot="carousel"
                data-orientation=orientation.as_str()
                aria-roledescription="carousel"
                // The arrows work from anywhere in the carousel, which is what makes it operable
                // without reaching for the two buttons.
                on:keydown=move |e: ev::KeyboardEvent| {
                    let horizontal = orientation == Orientation::Horizontal;
                    match (e.key().as_str(), horizontal) {
                        ("ArrowLeft", true) | ("ArrowUp", false) => {
                            e.prevent_default();
                            ctx.prev();
                        }
                        ("ArrowRight", true) | ("ArrowDown", false) => {
                            e.prevent_default();
                            ctx.next();
                        }
                        _ => {}
                    }
                }
                class=class
            >
                {children()}
            </section>
        </Provider>
    }
}

#[component]
pub fn CarouselContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_carousel();

    view! {
        <div
            node_ref=ctx.track_ref
            data-slot="carousel-content"
            data-orientation=ctx.orientation.as_str()
            aria-live="polite"
            on:scroll=move |_| ctx.sync_index()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselItemRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_carousel();

    // Counted rather than measured: the buttons need to know how many there are before anyone
    // has scrolled, and a slide knows when it arrives and when it goes.
    ctx.count.update(|count| *count += 1);
    on_cleanup(move || {
        let _ = ctx
            .count
            .try_update(|count| *count = count.saturating_sub(1));
    });

    view! {
        <div
            data-carousel-item=""
            data-slot="carousel-item"
            role="group"
            aria-roledescription="slide"
            class=class
        >
            {children()}
        </div>
    }
}

/// Where the two buttons differ: which way they go, and which end disables them.
#[derive(Clone, Copy, PartialEq)]
pub enum CarouselStep {
    Previous,
    Next,
}

#[component]
pub fn CarouselButtonRoot(
    step: CarouselStep,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_carousel();

    let available = Signal::derive(move || match step {
        CarouselStep::Previous => ctx.can_scroll_prev(),
        CarouselStep::Next => ctx.can_scroll_next(),
    });

    view! {
        <button
            type="button"
            data-slot=match step {
                CarouselStep::Previous => "carousel-previous",
                CarouselStep::Next => "carousel-next",
            }
            data-orientation=ctx.orientation.as_str()
            disabled=move || disabled.get() || !available.get()
            on:click=move |_| match step {
                CarouselStep::Previous => ctx.prev(),
                CarouselStep::Next => ctx.next(),
            }
            class=class
        >
            {children()}
        </button>
    }
}
