//! A sequence of events along an axis.
//!
//! **This is a list, not a scaled axis.** Events sit evenly spaced in document order and are
//! never positioned by their actual dates — an activity feed, a changelog, the steps of an order.
//! A timeline where March and December sit proportionally apart is arithmetic over a domain, and
//! belongs beside [`crate::primitives::chart`] rather than here; conflating the two would give
//! every caller a layout that lies about half its inputs.
//!
//! An `<ol>` of `<li>`, because that is what a sequence of events is. A reader on a screen reader
//! is told how many there are and which one they are on before any of this file's styling
//! matters, and [`TimelineState::Current`] writes `aria-current="step"`.
//!
//! **Items count themselves**, the way carousel slides do, so each one knows whether it is the
//! last. That is not a nicety: a connector drawn after the final event points at nothing, and
//! `:last-child` stops being true the moment a caller wraps an item in anything at all.

use crate::utils::types::Orientation;
use leptos::{context::Provider, prelude::*};

/// How far along the sequence an event is.
///
/// The default is [`Self::Complete`], because the common timeline is a record of things that have
/// already happened; a caller running one forwards as a set of steps sets all three itself.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimelineState {
    #[default]
    Complete,
    /// Where the reader is now. The only state that writes `aria-current`, and there should be
    /// one of it — this layer does not enforce that, since a finished sequence has none.
    Current,
    Upcoming,
}

impl TimelineState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Current => "current",
            Self::Upcoming => "upcoming",
        }
    }
}

/// Which side of the axis an event's content sits on.
///
/// One idea over both axes: before the axis means left when the timeline runs down and above when
/// it runs across. A whole timeline picks a side, and an alternating one flips it every other
/// event — derived from the item's own index rather than set per item, because a zig-zag a caller
/// keeps in step by hand stops zig-zagging the moment one event is inserted.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimelineSide {
    /// Before the axis: to its left when vertical, above it when horizontal.
    Start,
    /// After the axis, which is where content goes unless told otherwise.
    #[default]
    End,
}

impl TimelineSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            Self::Start => Self::End,
            Self::End => Self::Start,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TimelineContext {
    pub orientation: Orientation,
    /// Whether events alternate sides of the axis.
    pub alternate: bool,
    /// Which side content sits on, and the side event zero takes when alternating.
    pub side: TimelineSide,
    /// How many events there are, kept by the events themselves.
    pub count: RwSignal<usize>,
}

/// What the parts of one event share: where it sits, and what it is.
#[derive(Copy, Clone)]
pub struct TimelineItemContext {
    pub index: usize,
    pub state: Signal<TimelineState>,
    /// Whether this is the final event, so a connector knows it has nothing to point at.
    pub is_last: Signal<bool>,
    pub orientation: Orientation,
    /// Which side of the axis this event's content is on. See [`TimelineSide`].
    pub side: TimelineSide,
    pub alternate: bool,
}

pub fn use_timeline() -> TimelineContext {
    expect_context::<TimelineContext>()
}

pub fn use_timeline_item() -> TimelineItemContext {
    expect_context::<TimelineItemContext>()
}

#[component]
pub fn TimelineRoot(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    /// Whether events fall on alternating sides of the axis — left and right of a vertical
    /// timeline, above and below a horizontal one.
    #[prop(default = false)]
    alternate: bool,
    /// Which side of the axis content sits on, and where an alternating timeline starts.
    #[prop(optional)]
    side: TimelineSide,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = TimelineContext {
        orientation,
        alternate,
        side,
        count: RwSignal::new(0),
    };

    view! {
        <Provider value=ctx>
            <ol
                data-slot="timeline"
                data-orientation=orientation.as_str()
                data-alternate=move || if alternate { "true" } else { "false" }
                class=class
            >
                {children()}
            </ol>
        </Provider>
    }
}

#[component]
pub fn TimelineItemRoot(
    #[prop(optional, into)] state: Signal<TimelineState>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_timeline();

    // Counted rather than measured, and on the way in, so an item's own index is its position in
    // document order. Taken back on cleanup, or a list that loses an item keeps its old length
    // and every connector believes there is another event coming.
    let index = ctx.count.get_untracked();
    ctx.count.update(|count| *count += 1);
    on_cleanup(move || {
        let _ = ctx
            .count
            .try_update(|count| *count = count.saturating_sub(1));
    });

    let is_last = Signal::derive(move || index + 1 >= ctx.count.get());
    let side = match (ctx.alternate, index % 2) {
        (true, 1) => ctx.side.flip(),
        _ => ctx.side,
    };
    let item = TimelineItemContext {
        index,
        state,
        is_last,
        orientation: ctx.orientation,
        side,
        alternate: ctx.alternate,
    };

    view! {
        <Provider value=item>
            <li
                data-slot="timeline-item"
                data-side=side.as_str()
                data-alternate=move || if ctx.alternate { "true" } else { "false" }
                data-state=move || state.get().as_str()
                data-orientation=ctx.orientation.as_str()
                data-last=move || is_last.get().then_some("true")
                aria-current=move || (state.get() == TimelineState::Current).then_some("step")
                class=class
            >
                {children()}
            </li>
        </Provider>
    }
}

/// The dot, number or icon sitting on the axis.
///
/// What goes inside it is the caller's: a count, a tick, an avatar. This owns only where it sits
/// and what it says about itself.
#[component]
pub fn TimelineMarkerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let item = use_timeline_item();
    // Decorative unless the caller puts something in it: the state is already on the item, and a
    // screen reader reading "bullet" before every event helps nobody.
    let decorative = children.is_none().then_some("true");

    view! {
        <span
            data-slot="timeline-marker"
            data-side=item.side.as_str()
            data-alternate=move || if item.alternate { "true" } else { "false" }
            data-state=move || item.state.get().as_str()
            data-orientation=item.orientation.as_str()
            aria-hidden=decorative
            class=class
        >
            {children.map(|children| children())}
        </span>
    }
}

/// The line from this event to the next.
///
/// A part the caller places rather than something the item draws, so a pending stretch can be
/// dashed where a finished one is solid. It still knows when it is last, and the styled layer is
/// what decides that means hidden.
#[component]
pub fn TimelineConnectorRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let item = use_timeline_item();

    view! {
        <span
            data-slot="timeline-connector"
            data-side=item.side.as_str()
            data-alternate=move || if item.alternate { "true" } else { "false" }
            data-state=move || item.state.get().as_str()
            data-orientation=item.orientation.as_str()
            data-last=move || item.is_last.get().then_some("true")
            aria-hidden="true"
            class=class
        />
    }
}

#[component]
pub fn TimelineContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let item = use_timeline_item();

    view! {
        <div
            data-slot="timeline-content"
            data-side=item.side.as_str()
            data-alternate=move || if item.alternate { "true" } else { "false" }
            data-state=move || item.state.get().as_str()
            data-orientation=item.orientation.as_str()
            data-last=move || item.is_last.get().then_some("true")
            class=class
        >
            {children()}
        </div>
    }
}

/// When the event happened: a real `<time>`, with the machine-readable form in `datetime`.
///
/// The two are separate on purpose. What a reader wants to see is "3 days ago" or "14 March";
/// what a machine wants is `2026-03-14`, and neither is derivable from the other here — this
/// library does not parse or format dates for you beyond [`crate::utils::date`].
#[component]
pub fn TimelineTimeRoot(
    /// The `datetime` attribute. Left off, the element is still a `<time>` and still fine; it is
    /// only the machine-readable half that is missing.
    #[prop(optional, into)]
    datetime: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let item = use_timeline_item();

    view! {
        <time
            data-slot="timeline-time"
            data-state=move || item.state.get().as_str()
            datetime=move || {
                let datetime = datetime.get();
                (!datetime.is_empty()).then_some(datetime)
            }
            class=class
        >
            {children()}
        </time>
    }
}
