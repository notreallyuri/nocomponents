pub mod cn;
pub mod types;

#[cfg(feature = "highlight")]
pub mod highlight;

#[cfg(feature = "theme")]
pub mod theme;

use floating_ui_leptos::Placement;
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
