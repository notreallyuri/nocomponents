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
use leptos::{
    context::Provider,
    ev,
    prelude::*,
    wasm_bindgen::{JsCast, closure::Closure},
};
use leptos_node_ref::AnyNodeRef;
use web_sys::{HtmlElement, ResizeObserver};

/// The attribute that marks a slide, so the track can find them in document order.
pub const CAROUSEL_ITEM: &str = "data-carousel-item";

#[derive(Copy, Clone)]
pub struct CarouselContext {
    /// The slide at the start of the track.
    pub index: RwSignal<usize>,
    /// How many slides there are, kept by the slides themselves.
    pub count: RwSignal<usize>,
    /// The last slide the track can actually bring to its start, read back out of the track's own
    /// scroll extent by [`CarouselContext::sync`], and `None` until it has been measured once.
    ///
    /// The distinction matters: a bound of zero means "nowhere to go", and a carousel that has not
    /// been measured yet would wear it as one — a Next disabled before the first measurement is a
    /// Next that disabled the only gesture that would have measured it again.
    pub last: RwSignal<Option<usize>>,
    pub orientation: Orientation,
    /// Whether moving past either end comes round the other side.
    pub wrap: bool,
    /// The scroll container.
    pub track_ref: AnyNodeRef,
}

/// One read of the track's layout: see [`CarouselContext::measure`].
struct Measurements {
    /// Where each slide sits relative to the start of the track, in pixels of scroll.
    offsets: Vec<f64>,
    /// Where the track is scrolled to now.
    scrolled: f64,
    /// The furthest it can be scrolled.
    extent: f64,
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

    /// Everything a decision here needs, from one read of the layout: where each slide sits in
    /// pixels of scroll, where the track is scrolled to, and how far it can go.
    ///
    /// Read together because reading them apart means walking every slide twice on an event that
    /// arrives once a frame.
    fn measure(&self) -> Option<Measurements> {
        let track = self.track_ref.get_untracked()?;
        let rect = track.get_bounding_client_rect();

        let (start, scrolled, extent) = match self.orientation {
            Orientation::Horizontal => (
                rect.left(),
                track.scroll_left() as f64,
                (track.scroll_width() - track.client_width()) as f64,
            ),
            Orientation::Vertical => (
                rect.top(),
                track.scroll_top() as f64,
                (track.scroll_height() - track.client_height()) as f64,
            ),
        };

        let offsets = self
            .items()
            .iter()
            .map(|item| {
                let rect = item.get_bounding_client_rect();
                let edge = match self.orientation {
                    Orientation::Horizontal => rect.left(),
                    Orientation::Vertical => rect.top(),
                };
                edge - start + scrolled
            })
            .collect();

        Some(Measurements {
            offsets,
            scrolled,
            extent,
        })
    }

    /// Scrolls slide `index` to the start of the track. The scroll itself is animated by CSS.
    pub fn scroll_to(&self, index: usize) {
        let (Some(track), Some(m)) = (self.track_ref.get_untracked(), self.measure()) else {
            return;
        };
        let Some(&offset) = m.offsets.get(index) else {
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

    /// Reads the carousel's state back out of the track: which slide is current, and which is the
    /// last one the track can put at its start.
    ///
    /// The current slide is the one nearest the start of the track. The bound exists because a
    /// carousel showing three slides at once runs out of scroll with the last three on screen: the
    /// final few slides are all the same scroll position, so `count - 1` is a slide rather than
    /// somewhere the track can go, and a Next counting slides stays enabled for clicks that move
    /// nothing.
    pub fn sync(&self) {
        let Some(m) = self.measure() else {
            return;
        };
        if m.offsets.is_empty() {
            return;
        }

        let nearest = m
            .offsets
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                (*a - m.scrolled)
                    .abs()
                    .partial_cmp(&(*b - m.scrolled).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Loosely, because the last reachable slide lands *on* the extent and sub-pixel layout
        // puts it a fraction past about half the time.
        let last = m
            .offsets
            .iter()
            .rposition(|offset| *offset <= m.extent + 1.0)
            .unwrap_or(0);

        if self.index.get_untracked() != nearest {
            self.index.set(nearest);
        }
        if self.last.get_untracked() != Some(last) {
            self.last.set(Some(last));
        }
    }

    pub fn can_scroll_prev(&self) -> bool {
        self.wrap || self.index.get() > 0
    }

    /// The last slide the track can reach, or — until it has been measured — the last slide there
    /// is. Counting slides is what this got wrong, but it errs towards a click that moves nothing
    /// rather than towards a button that can never move anything.
    fn bound(&self) -> usize {
        self.last
            .get()
            .unwrap_or_else(|| self.count.get().saturating_sub(1))
    }

    fn bound_untracked(&self) -> usize {
        self.last
            .get_untracked()
            .unwrap_or_else(|| self.count.get_untracked().saturating_sub(1))
    }

    pub fn can_scroll_next(&self) -> bool {
        self.wrap || self.index.get() < self.bound()
    }

    pub fn prev(&self) {
        let count = self.count.get_untracked();
        if count == 0 {
            return;
        }
        let index = self.index.get_untracked();

        match (index, self.wrap) {
            // The end is the last *position*, not the last slide: wrapping backwards onto a slide
            // the track cannot reach would clamp to the same scroll and disagree about the index.
            (0, true) => self.scroll_to(self.bound_untracked()),
            (0, false) => {}
            _ => self.scroll_to(index - 1),
        }
    }

    pub fn next(&self) {
        if self.count.get_untracked() == 0 {
            return;
        }
        let index = self.index.get_untracked();

        if index < self.bound_untracked() {
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
        last: RwSignal::new(None),
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

    let observer = StoredValue::new_local(None::<ResizeObserver>);

    // How far the track can go depends on how much of it is on screen, which nothing reactive
    // reports: it changes when slides come and go, and when the track is resized without its
    // scroll position moving — the case no `scroll` event covers.
    //
    // The observer is what makes the bound self-correcting, and it is not a nicety: a bound
    // measured before the track has a box is zero, which disables Next, which is then the only
    // thing that could have scrolled the track and measured it again. Leaving a page and coming
    // back mounts one exactly that way.
    Effect::new(move |_| {
        ctx.count.track();
        let Some(track) = ctx.track_ref.get() else {
            return;
        };

        ctx.sync();

        if observer.with_value(|observer| observer.is_some()) {
            return;
        }

        let callback = Closure::<dyn Fn()>::new(move || ctx.sync());
        if let Ok(resize_observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            resize_observer.observe(&track);
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
            node_ref=ctx.track_ref
            data-slot="carousel-content"
            data-orientation=ctx.orientation.as_str()
            aria-live="polite"
            on:scroll=move |_| ctx.sync()
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
