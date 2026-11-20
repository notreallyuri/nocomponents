//! Pure pointer-gesture types and auto-scroll ramp math.

/// Where the pointer is, in each of the forms a caller has wanted.
#[derive(Clone, Copy, Debug, Default)]
pub struct DragPoint {
    pub client_x: f64,
    pub client_y: f64,
    /// Movement since the press. What a gesture that moves something *by* an amount wants.
    pub dx: f64,
    pub dy: f64,
    /// Where the pointer sits inside the reference box: 0 at its left or top edge, 1 at its right
    /// or bottom. Deliberately unclamped, so a caller can tell an overshoot from an edge — and
    /// measured live, so a page that scrolls mid-drag does not shift the frame under the gesture.
    pub fraction_x: f64,
    pub fraction_y: f64,
    /// The modifiers held for *this* move, not for the press that began the gesture — every
    /// pointer event carries its own, so taking or releasing a key partway through changes what
    /// the rest of the drag means without the caller listening for keys itself.
    ///
    /// Only the two a caller has wanted so far. `ctrl` and `meta` go here when something needs
    /// them; there is nothing to work out, only a line to add.
    pub shift: bool,
    pub alt: bool,
}

/// The release, and how the pointer was travelling when it came.
#[derive(Clone, Copy, Debug, Default)]
pub struct DragEnd {
    pub point: DragPoint,
    /// Pixels per millisecond over the last movement, signed. Zero if the pointer never moved.
    pub velocity_x: f64,
    pub velocity_y: f64,
    /// Milliseconds since the pointer last moved. A pause before release is a placement rather
    /// than a throw, which is the difference between dropping a panel and flicking it away.
    pub idle: f64,
}

/// How fast to scroll, from where a held pointer sits between the two edges of a scroller.
///
/// Nothing in the middle, and a ramp inside `edge` pixels of either end: zero at the inner limit
/// of the zone and `max` at the edge itself, so a scroller that lurches the moment the pointer
/// enters the zone is not the one being aimed at. Signed: negative scrolls towards `start`.
/// Past the edge it holds at `max` rather than growing, since a pointer can leave the box.
///
/// Pure, so the two things that auto-scroll (the kanban board, a sortable list) share one curve.
pub fn edge_ramp(position: f64, start: f64, end: f64, edge: f64, max: f64) -> f64 {
    if end <= start || edge <= 0.0 {
        return 0.0;
    }

    if position < start + edge {
        -((start + edge - position) / edge).clamp(0.0, 1.0) * max
    } else if position > end - edge {
        ((position - (end - edge)) / edge).clamp(0.0, 1.0) * max
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_scrolls_in_the_middle() {
        assert_eq!(edge_ramp(50.0, 0.0, 100.0, 20.0, 16.0), 0.0);
        assert_eq!(edge_ramp(20.0, 0.0, 100.0, 20.0, 16.0), 0.0);
        assert_eq!(edge_ramp(80.0, 0.0, 100.0, 20.0, 16.0), 0.0);
    }

    #[test]
    fn the_ramp_reaches_the_maximum_at_the_edge_and_holds_beyond_it() {
        assert_eq!(edge_ramp(0.0, 0.0, 100.0, 20.0, 16.0), -16.0);
        assert_eq!(edge_ramp(-30.0, 0.0, 100.0, 20.0, 16.0), -16.0);
        assert_eq!(edge_ramp(100.0, 0.0, 100.0, 20.0, 16.0), 16.0);
        assert_eq!(edge_ramp(140.0, 0.0, 100.0, 20.0, 16.0), 16.0);
    }

    #[test]
    fn the_ramp_is_proportional_inside_the_zone() {
        assert_eq!(edge_ramp(10.0, 0.0, 100.0, 20.0, 16.0), -8.0);
        assert_eq!(edge_ramp(90.0, 0.0, 100.0, 20.0, 16.0), 8.0);
    }

    #[test]
    fn an_empty_box_does_not_scroll() {
        assert_eq!(edge_ramp(5.0, 10.0, 10.0, 20.0, 16.0), 0.0);
        assert_eq!(edge_ramp(5.0, 0.0, 100.0, 0.0, 16.0), 0.0);
    }
}
