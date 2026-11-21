//! Long-press pointer-type filtering and drift math.

/// How long a press is held before it counts, in milliseconds.
pub const DEFAULT_DELAY_MS: f64 = 450.0;
/// How far, in pixels, the pointer may wander before the press is a drag or a scroll instead.
pub const MOVE_TOLERANCE: f64 = 10.0;

/// Where the held press went down, in viewport coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LongPressPoint {
    pub client_x: f64,
    pub client_y: f64,
}

/// Whether a pointer of this `PointerEvent.pointerType` can long-press. Mouse cannot.
pub fn can_long_press(pointer_type: &str) -> bool {
    matches!(pointer_type, "touch" | "pen")
}

/// Whether a pointer that has moved `(dx, dy)` from its press has gone further than `tolerance`
/// and so abandoned the press. Exactly at the tolerance is still a hold.
pub fn has_drifted(dx: f64, dy: f64, tolerance: f64) -> bool {
    dx * dx + dy * dy > tolerance * tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_and_pen_can_long_press_but_a_mouse_cannot() {
        assert!(can_long_press("touch"));
        assert!(can_long_press("pen"));
        assert!(!can_long_press("mouse"));
        assert!(!can_long_press(""));
    }

    #[test]
    fn a_press_within_the_tolerance_is_still_a_hold() {
        assert!(!has_drifted(0.0, 0.0, MOVE_TOLERANCE));
        assert!(!has_drifted(6.0, 8.0, MOVE_TOLERANCE));
        assert!(!has_drifted(-10.0, 0.0, MOVE_TOLERANCE));
    }

    #[test]
    fn a_press_beyond_the_tolerance_is_abandoned_in_any_direction() {
        assert!(has_drifted(11.0, 0.0, MOVE_TOLERANCE));
        assert!(has_drifted(0.0, -11.0, MOVE_TOLERANCE));
        assert!(has_drifted(8.0, 8.0, MOVE_TOLERANCE));
    }
}
