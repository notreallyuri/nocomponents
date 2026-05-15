pub mod cn;
pub mod types;

#[cfg(feature = "theme")]
pub mod theme;

use floating_ui_leptos::Placement;
use types::{Align, Side};

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
