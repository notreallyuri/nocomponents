pub mod cn;
pub mod color;
pub mod date;
pub mod gif;
pub mod image;
pub mod types;
pub mod viewport;

#[cfg(feature = "highlight")]
pub mod highlight;

#[cfg(feature = "theme")]
pub mod theme;

use floating_ui_leptos::Placement;
use leptos::{ev, prelude::*};
use std::cell::Cell;
use types::{Align, Side};

thread_local! {
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
}

/// A document-unique id, for the `aria-*` attributes that have to point at another element.
///
/// Components mint their own rather than asking callers for one: `aria-labelledby` needs an id on
/// the label whether or not anybody wanted to write one.
pub fn next_id(prefix: &str) -> String {
    let n = NEXT_ID.with(|next| {
        let n = next.get();
        next.set(n + 1);
        n
    });

    format!("nc-{prefix}-{n}")
}

/// Whether a CSS media query matches, kept up to date as the window changes.
///
/// For the handful of decisions a class cannot make — a dialog that has to be a drawer on a
/// phone is a different *component*, not a different width — where the alternative is rendering
/// both and hiding one, which leaves two of everything in the DOM and two of everything focusable.
///
/// Listens for `resize` rather than to the `MediaQueryList` itself: the query is re-read on each
/// event, so what this says and what CSS is doing cannot drift apart.
pub fn use_media_query(query: &'static str) -> Signal<bool> {
    let matches = RwSignal::new(query_matches(query));

    let listener = window_event_listener(ev::resize, move |_| {
        let now = query_matches(query);
        if matches.get_untracked() != now {
            matches.set(now);
        }
    });

    on_cleanup(move || listener.remove());

    matches.into()
}

fn query_matches(query: &str) -> bool {
    window()
        .match_media(query)
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}

pub fn get_placement(side: Side, align: Align) -> Placement {
    match (side, align) {
        (Side::Top, Align::Start) => Placement::TopStart,
        (Side::Top, Align::Center) => Placement::Top,
        (Side::Top, Align::End) => Placement::TopEnd,
        (Side::Bottom, Align::Start) => Placement::BottomStart,
        (Side::Bottom, Align::Center) => Placement::Bottom,
        (Side::Bottom, Align::End) => Placement::BottomEnd,
        (Side::Left, Align::Start) => Placement::LeftStart,
        (Side::Left, Align::Center) => Placement::Left,
        (Side::Left, Align::End) => Placement::LeftEnd,
        (Side::Right, Align::Start) => Placement::RightStart,
        (Side::Right, Align::Center) => Placement::Right,
        (Side::Right, Align::End) => Placement::RightEnd,
    }
}
