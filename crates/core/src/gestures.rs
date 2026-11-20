//! Pure touch and pointer gesture classification for media viewers.
//!
//! Covers which third of the screen a tap is in, double taps, horizontal/vertical drag and swipe
//! classification, release actions, and overlay auto-hide state. Times are milliseconds, distances
//! CSS pixels.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TapZone {
    Left,
    Center,
    Right,
}

/// Thirds of the media area: the sides seek on a double tap in a video.
pub fn tap_zone(x: f64, width: f64) -> TapZone {
    if x < width / 3.0 {
        TapZone::Left
    } else if x > width * 2.0 / 3.0 {
        TapZone::Right
    } else {
        TapZone::Center
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tap {
    pub time: f64,
    pub zone: TapZone,
}

pub const DOUBLE_TAP_MS: f64 = 280.0;

/// A second tap on the same side, soon after the first. Videos only seek on the sides; images
/// zoom anywhere (`any_zone`).
pub fn is_double_tap(previous: Option<Tap>, current: Tap, any_zone: bool) -> bool {
    previous.is_some_and(|previous| {
        (any_zone || current.zone != TapZone::Center)
            && current.zone == previous.zone
            && current.time - previous.time <= DOUBLE_TAP_MS
    })
}

/// A press becomes a horizontal drag once it moves sideways more than this.
pub const DRAG_START_PX: f64 = 12.0;
/// How far a quick flick must go.
pub const SWIPE_PX: f64 = 60.0;
/// A flick slower than this is a drag, not a swipe.
pub const SWIPE_MAX_MS: f64 = 800.0;
/// A mouse may move this far between press and release and still count as a click.
pub const MOUSE_CLICK_SLOP_PX: f64 = 8.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Browse {
    Next,
    Previous,
}

pub fn is_horizontal_drag(dx: f64, dy: f64) -> bool {
    dx.abs() > DRAG_START_PX && dx.abs() > dy.abs()
}

/// A press that moves up or down more than sideways and more than the slop: the start of a
/// vertical swipe.
pub fn is_vertical_drag(dx: f64, dy: f64) -> bool {
    dy.abs() > DRAG_START_PX && dy.abs() > dx.abs()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vertical {
    /// A flick up.
    Up,
    /// A flick down.
    Down,
}

/// A clear vertical flick: far enough, mostly vertical, and quick.
pub fn vertical_swipe(dx: f64, dy: f64, duration: f64) -> Option<Vertical> {
    if duration > SWIPE_MAX_MS || dy.abs() < SWIPE_PX || dy.abs() < dx.abs() * 1.5 {
        return None;
    }
    Some(if dy < 0.0 {
        Vertical::Up
    } else {
        Vertical::Down
    })
}

/// A clear sideways flick: far enough, mostly horizontal, and quick.
pub fn classify_swipe(dx: f64, dy: f64, duration: f64) -> Option<Browse> {
    if duration > SWIPE_MAX_MS || dx.abs() < SWIPE_PX || dx.abs() < dy.abs() * 1.5 {
        return None;
    }
    Some(if dx < 0.0 {
        Browse::Next
    } else {
        Browse::Previous
    })
}

/// Where a released sideways drag goes: past about a fifth of the width, or a quick swipe, moves to
/// the neighbour; anything else springs back.
pub fn settle_drag(dx: f64, dy: f64, duration: f64, width: f64) -> Option<Browse> {
    if dx.abs() > width * 0.2 && dx.abs() > dy.abs() {
        return Some(if dx < 0.0 {
            Browse::Next
        } else {
            Browse::Previous
        });
    }
    classify_swipe(dx, dy, duration)
}

/// What a press on the surface has turned out to be so far.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragKind {
    /// Still, or moved less than the slop: a tap if it ends here.
    Undecided,
    Horizontal,
    Vertical,
}

/// The kind of a press after it moved to `(dx, dy)` from where it began. A kind, once decided,
/// stays: a drag that wanders back to the middle is still that drag.
pub fn drag_kind(current: DragKind, dx: f64, dy: f64) -> DragKind {
    if current != DragKind::Undecided {
        current
    } else if is_horizontal_drag(dx, dy) {
        DragKind::Horizontal
    } else if is_vertical_drag(dx, dy) {
        DragKind::Vertical
    } else {
        DragKind::Undecided
    }
}

/// What lifting the finger (or releasing the mouse) means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Release {
    /// A sideways drag that goes on to a neighbour.
    Browse(Browse),
    /// A sideways drag that did not go far enough: back to rest.
    SpringBack,
    /// A vertical flick.
    Vertical(Vertical),
    /// A vertical drag that was not a flick: nothing.
    Nothing,
    /// A mouse click on the surface: play or pause.
    Click,
    /// A touch tap: shows or hides the overlay (or, twice on a side, seeks). Never a swipe or a drag.
    Tap,
}

/// The one place where a press that ended becomes a tap, a swipe, or nothing, so that a swipe can
/// never also toggle the overlay and a tap never browses. `width` is the media area's width.
pub fn classify_release(
    kind: DragKind,
    dx: f64,
    dy: f64,
    duration: f64,
    width: f64,
    mouse: bool,
) -> Release {
    match kind {
        DragKind::Horizontal => {
            settle_drag(dx, dy, duration, width).map_or(Release::SpringBack, Release::Browse)
        }
        DragKind::Vertical => {
            vertical_swipe(dx, dy, duration).map_or(Release::Nothing, Release::Vertical)
        }
        DragKind::Undecided if mouse => {
            if dx.hypot(dy) <= MOUSE_CLICK_SLOP_PX {
                Release::Click
            } else {
                Release::Nothing
            }
        }
        DragKind::Undecided => Release::Tap,
    }
}

/// How long the overlay stays up after the last touch while the video plays.
pub const AUTO_HIDE_MS: u64 = 2500;

/// The overlay is held up whatever the auto-hide says: a paused video, a seek drag in progress, or
/// an open menu.
pub fn overlay_pinned(playing: bool, scrubbing: bool, menu_open: bool) -> bool {
    !playing || scrubbing || menu_open
}

/// Whether the overlay is on screen: it was shown (a tap, a touch of a control, a mouse move) or
/// it is pinned.
pub fn overlay_visible(shown: bool, playing: bool, scrubbing: bool, menu_open: bool) -> bool {
    shown || overlay_pinned(playing, scrubbing, menu_open)
}

/// Whether the auto-hide timer may hide the overlay when it runs out.
pub fn auto_hide_due(playing: bool, scrubbing: bool, menu_open: bool) -> bool {
    !overlay_pinned(playing, scrubbing, menu_open)
}

/// What a tap on the video makes of the overlay: shown when it was away, away when it was shown.
pub fn shown_after_tap(visible: bool) -> bool {
    !visible
}

/// Only a mouse moving over the player shows the overlay, and only a mouse leaving it hides it at
/// once: a finger lifting is also a pointer leaving, and it must not hide what was just tapped.
pub fn is_mouse(pointer_type: &str) -> bool {
    pointer_type == "mouse"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tap_lands_in_a_third() {
        assert_eq!(tap_zone(10.0, 300.0), TapZone::Left);
        assert_eq!(tap_zone(150.0, 300.0), TapZone::Center);
        assert_eq!(tap_zone(290.0, 300.0), TapZone::Right);
        assert_eq!(tap_zone(100.0, 300.0), TapZone::Center);
    }

    #[test]
    fn a_double_tap_is_two_quick_taps_on_the_same_side() {
        let a = Tap {
            time: 1000.0,
            zone: TapZone::Left,
        };
        assert!(is_double_tap(
            Some(a),
            Tap {
                time: 1200.0,
                zone: TapZone::Left
            },
            false
        ));
        assert!(!is_double_tap(
            Some(a),
            Tap {
                time: 1400.0,
                zone: TapZone::Left
            },
            false
        ));
        assert!(!is_double_tap(
            Some(a),
            Tap {
                time: 1100.0,
                zone: TapZone::Right
            },
            false
        ));
        assert!(!is_double_tap(None, a, false));
        // The middle counts for images only.
        let c = Tap {
            time: 1000.0,
            zone: TapZone::Center,
        };
        assert!(!is_double_tap(
            Some(c),
            Tap {
                time: 1100.0,
                zone: TapZone::Center
            },
            false
        ));
        assert!(is_double_tap(
            Some(c),
            Tap {
                time: 1100.0,
                zone: TapZone::Center
            },
            true
        ));
    }

    #[test]
    fn a_drag_starts_once_it_moves_sideways_enough() {
        assert!(!is_horizontal_drag(10.0, 0.0));
        assert!(is_horizontal_drag(-13.0, 4.0));
        assert!(!is_horizontal_drag(20.0, 30.0));
    }

    #[test]
    fn swipes_need_distance_speed_and_a_mostly_sideways_path() {
        assert_eq!(classify_swipe(-80.0, 10.0, 200.0), Some(Browse::Next));
        assert_eq!(classify_swipe(80.0, 10.0, 200.0), Some(Browse::Previous));
        assert_eq!(classify_swipe(-59.0, 0.0, 100.0), None);
        assert_eq!(classify_swipe(-80.0, 0.0, 900.0), None);
        assert_eq!(classify_swipe(-80.0, 70.0, 100.0), None);
    }

    #[test]
    fn a_released_drag_goes_on_past_a_fifth_of_the_width() {
        assert_eq!(settle_drag(-90.0, 0.0, 2000.0, 400.0), Some(Browse::Next));
        assert_eq!(
            settle_drag(90.0, 0.0, 2000.0, 400.0),
            Some(Browse::Previous)
        );
        assert_eq!(settle_drag(-50.0, 0.0, 2000.0, 400.0), None);
        // Short but quick: a flick.
        assert_eq!(settle_drag(-65.0, 0.0, 150.0, 400.0), Some(Browse::Next));
        // More down than across never browses.
        assert_eq!(settle_drag(-90.0, 200.0, 2000.0, 400.0), None);
    }

    #[test]
    fn vertical_flicks_step_releases() {
        assert!(is_vertical_drag(2.0, 20.0));
        assert!(is_vertical_drag(2.0, -20.0));
        assert!(!is_vertical_drag(30.0, 20.0));
        assert_eq!(vertical_swipe(5.0, -80.0, 200.0), Some(Vertical::Up));
        assert_eq!(vertical_swipe(5.0, 80.0, 200.0), Some(Vertical::Down));
        assert_eq!(vertical_swipe(0.0, -59.0, 100.0), None);
        assert_eq!(vertical_swipe(0.0, -80.0, 900.0), None);
        assert_eq!(vertical_swipe(60.0, -80.0, 100.0), None);
    }

    #[test]
    fn a_press_decides_its_kind_once() {
        assert_eq!(
            drag_kind(DragKind::Undecided, 5.0, 5.0),
            DragKind::Undecided
        );
        assert_eq!(
            drag_kind(DragKind::Undecided, -20.0, 3.0),
            DragKind::Horizontal
        );
        assert_eq!(
            drag_kind(DragKind::Undecided, 3.0, 20.0),
            DragKind::Vertical
        );
        // Back near the start, or turned the other way: it stays what it was.
        assert_eq!(
            drag_kind(DragKind::Horizontal, 1.0, 40.0),
            DragKind::Horizontal
        );
        assert_eq!(drag_kind(DragKind::Vertical, 40.0, 1.0), DragKind::Vertical);
    }

    #[test]
    fn a_touch_that_did_not_move_is_a_tap_and_a_drag_is_never_one() {
        assert_eq!(
            classify_release(DragKind::Undecided, 3.0, 2.0, 60.0, 390.0, false),
            Release::Tap
        );
        // A long press that stayed put is still a tap.
        assert_eq!(
            classify_release(DragKind::Undecided, 0.0, 0.0, 900.0, 390.0, false),
            Release::Tap
        );
        assert_eq!(
            classify_release(DragKind::Horizontal, -200.0, 5.0, 180.0, 390.0, false),
            Release::Browse(Browse::Next)
        );
        assert_eq!(
            classify_release(DragKind::Horizontal, 150.0, 5.0, 180.0, 390.0, false),
            Release::Browse(Browse::Previous)
        );
        assert_eq!(
            classify_release(DragKind::Horizontal, 30.0, 2.0, 1500.0, 390.0, false),
            Release::SpringBack
        );
        assert_eq!(
            classify_release(DragKind::Vertical, 4.0, -90.0, 150.0, 390.0, false),
            Release::Vertical(Vertical::Up)
        );
        assert_eq!(
            classify_release(DragKind::Vertical, 4.0, 90.0, 150.0, 390.0, false),
            Release::Vertical(Vertical::Down)
        );
        assert_eq!(
            classify_release(DragKind::Vertical, 4.0, -30.0, 150.0, 390.0, false),
            Release::Nothing
        );
        for kind in [DragKind::Horizontal, DragKind::Vertical] {
            for (dx, dy) in [(0.0, 0.0), (-300.0, 0.0), (0.0, -300.0), (20.0, 20.0)] {
                assert_ne!(
                    classify_release(kind, dx, dy, 100.0, 390.0, false),
                    Release::Tap
                );
            }
        }
    }

    #[test]
    fn a_mouse_click_plays_and_a_mouse_drag_does_nothing() {
        assert_eq!(
            classify_release(DragKind::Undecided, 2.0, 3.0, 80.0, 1000.0, true),
            Release::Click
        );
        assert_eq!(
            classify_release(DragKind::Undecided, 30.0, 0.0, 80.0, 1000.0, true),
            Release::Nothing
        );
        assert!(is_mouse("mouse"));
        assert!(!is_mouse("touch") && !is_mouse("pen"));
    }

    #[test]
    fn the_overlay_is_held_by_a_pause_a_scrub_or_a_menu() {
        assert!(overlay_visible(false, false, false, false));
        assert!(overlay_visible(false, true, true, false));
        assert!(overlay_visible(false, true, false, true));
        assert!(!overlay_visible(false, true, false, false));
        assert!(overlay_visible(true, true, false, false));
        assert!(auto_hide_due(true, false, false));
        assert!(!auto_hide_due(false, false, false));
        assert!(!auto_hide_due(true, true, false));
        assert!(!auto_hide_due(true, false, true));
    }

    #[test]
    fn one_tap_changes_the_overlay_once_whatever_it_showed() {
        // Playing, overlay away: the tap shows it; a second, later, tap hides it.
        let visible = |shown| overlay_visible(shown, true, false, false);
        let mut shown = false;
        shown = shown_after_tap(visible(shown));
        assert!(shown);
        shown = shown_after_tap(visible(shown));
        assert!(!shown);
        // Paused: always visible, so nothing changes on screen whatever the tap decides.
        assert!(overlay_visible(shown_after_tap(true), false, false, false));
    }
}
