//! Viewport clamping for a draggable floating panel.

/// How much of the panel must stay on screen. Enough to grab it back by.
pub const KEEP_VISIBLE: f64 = 32.0;

/// Keeps `position` far enough inside `view` that the panel can still be reached.
///
/// Against the *window* rather than the document: a panel fixed to the viewport is put where the
/// reader can see it, and a page that scrolls afterwards must not take it away.
///
/// Only the panel's width is wanted. It may hang off either side with a grabbable strip still
/// showing, and off the bottom the same way — but never off the top, because a handle dragged
/// above the top edge has nothing above it to reach for and cannot be brought back.
pub fn clamp(position: (f64, f64), width: f64, view: (f64, f64)) -> (f64, f64) {
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
