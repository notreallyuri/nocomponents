//! Where a thing you have let go of should come to rest.
//!
//! The drawer's drag and the phone sheets' drag decide the same way — how far, how fast, which of
//! a few resting heights is nearest — so the decision is arithmetic that lives here, apart from
//! either, and is unit-tested without a browser.

/// How recently the pointer must have moved for a release to count as a flick at all, in
/// milliseconds.
pub const FLICK_WINDOW: f64 = 100.0;
/// How far a panel with no snap points must travel, as a fraction of its own length, before a
/// release dismisses it.
pub const DEFAULT_DISTANCE_THRESHOLD: f64 = 0.25;
/// Pixels per millisecond past which a release on a panel with no snap points is a flick,
/// whatever the distance.
pub const DEFAULT_FLICK_VELOCITY: f64 = 0.4;

/// Below this share of the viewport, along the drawer's axis, a released snapping drawer is
/// dismissed.
pub const DEFAULT_DISMISS_BELOW: f64 = 0.22;
/// Pixels per millisecond past which a release on a snapping drawer moves one snap point.
pub const DEFAULT_SNAP_FLICK_VELOCITY: f64 = 0.6;

/// Where a released snapping drawer comes to rest.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Settle {
    /// Rest at this snap point.
    Snap(f64),
    Dismiss,
}

/// Decides where a released drawer goes.
///
/// - `position` is how much of the viewport the drawer covers along its axis at the release, as a
///   fraction: the same unit as `snap_points`, so `0.4` is a panel 40 percent of the way across.
/// - `velocity` is the release speed in pixels per millisecond, positive toward dismissal (the
///   panel shrinking) and zero for a release after a pause.
/// - `dismiss_below` is the position under which the drawer is thrown away however it got there.
/// - `flick_threshold` is the speed, in either direction, that counts as a flick.
///
/// Below `dismiss_below` it dismisses; a flick moves one snap point from `position` in its
/// direction, past the lowest dismissing and past the highest staying on it; anything slower goes
/// to the nearest. With no snap points there is a single rest at the full extent, `Snap(1.0)`.
pub fn settle(
    position: f64,
    velocity: f64,
    snap_points: &[f64],
    dismiss_below: f64,
    flick_threshold: f64,
) -> Settle {
    const EPSILON: f64 = 1e-9;

    let mut points: Vec<f64> = snap_points.to_vec();
    if points.is_empty() {
        points.push(1.0);
    }
    points.sort_by(|a, b| a.total_cmp(b));

    if position < dismiss_below {
        return Settle::Dismiss;
    }

    if velocity > flick_threshold {
        return match points
            .iter()
            .rev()
            .find(|point| **point < position - EPSILON)
        {
            Some(point) => Settle::Snap(*point),
            None => Settle::Dismiss,
        };
    }

    if velocity < -flick_threshold {
        return match points.iter().find(|point| **point > position + EPSILON) {
            Some(point) => Settle::Snap(*point),
            None => Settle::Snap(points[points.len() - 1]),
        };
    }

    let nearest = points
        .iter()
        .min_by(|a, b| (position - **a).abs().total_cmp(&(position - **b).abs()))
        .copied()
        .unwrap_or(1.0);
    Settle::Snap(nearest)
}

/// What a drag measured by the time it was let go.
#[derive(Clone, Copy, Debug, Default)]
pub struct Release {
    pub travel: f64,
    pub length: f64,
    pub extent: f64,
    pub current: Option<f64>,
    pub velocity: f64,
}

/// Decides where a panel dragged away from its edge goes when released, in the pixels the gesture
/// was measured in.
///
/// - `travel` is how far it was dragged, positive away from the anchored edge, and `length` its
///   own size along that axis.
/// - `extent` is the viewport along the axis, which snap points are fractions of, and `current`
///   the snap point it started at (`None` is the first). With no `snap_points` both are ignored:
///   the panel has one rest, and it is dismissed past [`DEFAULT_DISTANCE_THRESHOLD`] of its length
///   or by a flick of [`DEFAULT_FLICK_VELOCITY`], and a drag of nothing at all goes back.
/// - `velocity` is the release speed along the axis, positive toward dismissal and zero for a
///   release after a pause.
///
/// With snap points this is [`settle`] on the position the panel was dragged to.
pub fn settle_drag(
    release: Release,
    snap_points: &[f64],
    dismiss_below: f64,
    flick_threshold: f64,
) -> Settle {
    let Release {
        travel,
        length,
        extent,
        current,
        velocity,
    } = release;

    if snap_points.is_empty() {
        let far_enough = length > 0.0 && travel / length > DEFAULT_DISTANCE_THRESHOLD;
        let fast_enough = velocity > DEFAULT_FLICK_VELOCITY && travel > 0.0;
        return if far_enough || fast_enough {
            Settle::Dismiss
        } else {
            Settle::Snap(1.0)
        };
    }

    let current = current.unwrap_or(snap_points[0]);
    let position = if extent > 0.0 {
        current - travel / extent
    } else {
        current
    };
    settle(
        position,
        velocity,
        snap_points,
        dismiss_below,
        flick_threshold,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: [f64; 2] = [0.40, 0.88];
    const BELOW: f64 = 0.22;
    const FLICK: f64 = 0.6;

    fn settled(position: f64, velocity: f64) -> Settle {
        settle(position, velocity, &POINTS, BELOW, FLICK)
    }

    #[test]
    fn a_slow_release_goes_to_the_nearest_snap() {
        assert_eq!(settled(0.45, 0.0), Settle::Snap(0.40));
        assert_eq!(settled(0.70, 0.0), Settle::Snap(0.88));
        assert_eq!(settled(0.64, 0.0), Settle::Snap(0.40));
        assert_eq!(settled(0.65, 0.1), Settle::Snap(0.88));
        assert_eq!(settled(0.95, -0.2), Settle::Snap(0.88));
    }

    #[test]
    fn a_slow_release_between_the_floor_and_the_lowest_snap_returns_to_it() {
        assert_eq!(settled(0.25, 0.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_flick_up_moves_one_snap() {
        assert_eq!(settled(0.42, -1.0), Settle::Snap(0.88));
        assert_eq!(settled(0.40, -0.7), Settle::Snap(0.88));
    }

    #[test]
    fn a_flick_up_from_the_highest_stays_on_it() {
        assert_eq!(settled(0.88, -2.0), Settle::Snap(0.88));
        assert_eq!(settled(0.90, -2.0), Settle::Snap(0.88));
    }

    #[test]
    fn a_flick_down_moves_one_snap() {
        assert_eq!(settled(0.86, 1.0), Settle::Snap(0.40));
        assert_eq!(settled(0.88, 0.7), Settle::Snap(0.40));
        assert_eq!(settled(0.45, 1.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_flick_down_from_the_lowest_dismisses() {
        assert_eq!(settled(0.40, 1.0), Settle::Dismiss);
        assert_eq!(settled(0.35, 0.9), Settle::Dismiss);
    }

    #[test]
    fn a_release_below_the_floor_dismisses_however_slowly() {
        assert_eq!(settled(0.21, 0.0), Settle::Dismiss);
        assert_eq!(settled(0.10, -0.1), Settle::Dismiss);
        assert_eq!(settled(0.21, -5.0), Settle::Dismiss);
        assert_eq!(settled(0.22, 0.0), Settle::Snap(0.40));
    }

    #[test]
    fn a_speed_at_the_threshold_is_not_yet_a_flick() {
        assert_eq!(settled(0.45, FLICK), Settle::Snap(0.40));
        assert_eq!(settled(0.70, -FLICK), Settle::Snap(0.88));
    }

    #[test]
    fn the_order_the_points_are_given_in_does_not_matter() {
        let reversed = [0.88, 0.40];
        assert_eq!(
            settle(0.42, -1.0, &reversed, BELOW, FLICK),
            Settle::Snap(0.88)
        );
        assert_eq!(settle(0.40, 1.0, &reversed, BELOW, FLICK), Settle::Dismiss);
    }

    #[test]
    fn a_single_snap_is_a_rest_that_a_flick_down_dismisses() {
        assert_eq!(settle(0.6, 0.0, &[0.6], BELOW, FLICK), Settle::Snap(0.6));
        assert_eq!(settle(0.6, 0.9, &[0.6], BELOW, FLICK), Settle::Dismiss);
        assert_eq!(settle(0.6, -0.9, &[0.6], BELOW, FLICK), Settle::Snap(0.6));
    }

    #[test]
    fn no_snap_points_is_one_rest_at_the_full_extent() {
        assert_eq!(settle(0.97, 0.0, &[], BELOW, FLICK), Settle::Snap(1.0));
        assert_eq!(settle(0.5, 0.0, &[], BELOW, FLICK), Settle::Snap(1.0));
        assert_eq!(settle(0.5, 0.9, &[], BELOW, FLICK), Settle::Dismiss);
        assert_eq!(settle(0.1, 0.0, &[], BELOW, FLICK), Settle::Dismiss);
    }

    #[test]
    fn a_panel_without_snap_points_is_dismissed_past_a_quarter_of_its_length() {
        let go = |travel, velocity| {
            settle_drag(
                Release {
                    travel,
                    length: 400.0,
                    extent: 800.0,
                    velocity,
                    ..Release::default()
                },
                &[],
                0.0,
                0.0,
            )
        };
        assert_eq!(go(101.0, 0.0), Settle::Dismiss);
        assert_eq!(go(99.0, 0.0), Settle::Snap(1.0));
        assert_eq!(go(0.0, 0.0), Settle::Snap(1.0));
    }

    #[test]
    fn a_panel_without_snap_points_is_dismissed_by_a_flick_however_short() {
        let go = |travel, velocity| {
            settle_drag(
                Release {
                    travel,
                    length: 400.0,
                    extent: 800.0,
                    velocity,
                    ..Release::default()
                },
                &[],
                0.0,
                0.0,
            )
        };
        assert_eq!(go(10.0, 0.5), Settle::Dismiss);
        assert_eq!(go(10.0, 0.4), Settle::Snap(1.0));
        assert_eq!(go(0.0, 2.0), Settle::Snap(1.0));
        assert_eq!(go(10.0, -2.0), Settle::Snap(1.0));
    }

    #[test]
    fn a_dragged_snapping_panel_settles_on_where_it_was_let_go() {
        let go = |current, travel, velocity| {
            settle_drag(
                Release {
                    travel,
                    length: 700.0,
                    extent: 800.0,
                    current: Some(current),
                    velocity,
                },
                &POINTS,
                BELOW,
                FLICK,
            )
        };
        assert_eq!(go(0.88, 400.0, 0.0), Settle::Snap(0.40));
        assert_eq!(go(0.40, 100.0, 0.0), Settle::Snap(0.40));
        assert_eq!(go(0.40, 240.0, 0.0), Settle::Dismiss);
        assert_eq!(go(0.40, 20.0, 1.0), Settle::Dismiss);
        assert_eq!(go(0.40, -300.0, 0.0), Settle::Snap(0.88));
    }

    #[test]
    fn a_snapping_panel_with_no_current_starts_at_the_first_point_as_given() {
        let go = |travel| {
            settle_drag(
                Release {
                    travel,
                    length: 700.0,
                    extent: 800.0,
                    ..Release::default()
                },
                &[0.88, 0.40],
                BELOW,
                FLICK,
            )
        };
        assert_eq!(go(0.0), Settle::Snap(0.88));
    }
}
