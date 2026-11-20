//! Zoom and pan for fitted media as pure math.
//!
//! The picture is laid out centred in the view at its fitted size; a [`Zoom`] is a scale about the
//! picture's centre plus a shift of that centre from the view's centre. Points are measured from
//! the view's centre too, so the same formula serves wheel, pinch, and double tap.

/// The most a picture zooms in by default.
pub const MAX_ZOOM: f64 = 6.0;
/// A double tap zooms at least this far by default, more when the picture can fill the screen.
pub const MIN_TAP_ZOOM: f64 = 3.5;
/// Above this scale the picture counts as zoomed in by default (swiping and closing are off, panning is on).
pub const ZOOMED_ABOVE: f64 = 1.01;
/// Default wheel zoom sensitivity: the scale changes by `e^(-delta * step)`, about 22% for one notch.
pub const WHEEL_STEP: f64 = 0.002;

/// Dimensions in CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// Coordinates measured from the view's centre.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Current zoom scale and translation from the view's centre.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zoom {
    pub scale: f64,
    /// Where the picture's centre is, from the view's centre.
    pub x: f64,
    pub y: f64,
}

/// Tunable bounds and sensitivity for [`Zoom`] operations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomConfig {
    pub max_zoom: f64,
    pub min_tap_zoom: f64,
    pub zoomed_above: f64,
    pub wheel_step: f64,
}

impl Default for ZoomConfig {
    fn default() -> Self {
        Self {
            max_zoom: MAX_ZOOM,
            min_tap_zoom: MIN_TAP_ZOOM,
            zoomed_above: ZOOMED_ABOVE,
            wheel_step: WHEEL_STEP,
        }
    }
}

impl Zoom {
    pub const REST: Zoom = Zoom {
        scale: 1.0,
        x: 0.0,
        y: 0.0,
    };

    pub fn is_zoomed(&self) -> bool {
        self.is_zoomed_with(&ZoomConfig::default())
    }

    pub fn is_zoomed_with(&self, config: &ZoomConfig) -> bool {
        self.scale > config.zoomed_above
    }

    /// The CSS transform (the element's transform origin is its centre).
    pub fn css(&self) -> String {
        format!(
            "translate({:.2}px, {:.2}px) scale({:.4})",
            self.x, self.y, self.scale
        )
    }
}

/// The size `item` shows at when scaled to fit inside `view`, keeping its shape.
pub fn fit_within(item: Size, view: Size) -> Option<Size> {
    if item.width <= 0.0 || item.height <= 0.0 || view.width <= 0.0 || view.height <= 0.0 {
        return None;
    }
    let scale = (view.width / item.width).min(view.height / item.height);
    Some(Size {
        width: (item.width * scale).round(),
        height: (item.height * scale).round(),
    })
}

/// How far the centre may move along one axis: until the picture's edge meets the view's edge; a
/// picture smaller than the view stays centred.
fn limit(fitted: f64, scale: f64, view: f64) -> f64 {
    ((fitted * scale - view) / 2.0).max(0.0)
}

/// The zoom kept in bounds using `config`: a scale of 1 to `config.max_zoom`, and no empty space
/// beside a picture that is larger than the view.
pub fn clamp_with(zoom: Zoom, fitted: Size, view: Size, config: &ZoomConfig) -> Zoom {
    let max_zoom = config.max_zoom.max(1.0);
    let scale = if zoom.scale.is_finite() {
        zoom.scale.clamp(1.0, max_zoom)
    } else {
        1.0
    };
    let (lx, ly) = (
        limit(fitted.width, scale, view.width),
        limit(fitted.height, scale, view.height),
    );
    let shift = |value: f64, limit: f64| {
        if value.is_finite() {
            value.clamp(-limit, limit)
        } else {
            0.0
        }
    };
    Zoom {
        scale,
        x: shift(zoom.x, lx),
        y: shift(zoom.y, ly),
    }
}

/// The zoom kept in bounds: a scale of 1 to `MAX_ZOOM`, and no empty space beside a picture that
/// is larger than the view.
pub fn clamp(zoom: Zoom, fitted: Size, view: Size) -> Zoom {
    clamp_with(zoom, fitted, view, &ZoomConfig::default())
}

/// Zooms to `scale` using `config`, keeping the picture point under `at` (from the view's centre)
/// where it is.
pub fn zoom_at_with(
    zoom: Zoom,
    at: Point,
    scale: f64,
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    let max_zoom = config.max_zoom.max(1.0);
    let scale = if scale.is_finite() {
        scale.clamp(1.0, max_zoom)
    } else {
        1.0
    };
    let k = scale / zoom.scale;
    clamp_with(
        Zoom {
            scale,
            x: at.x - (at.x - zoom.x) * k,
            y: at.y - (at.y - zoom.y) * k,
        },
        fitted,
        view,
        config,
    )
}

/// Zooms to `scale` keeping the picture point under `at` (from the view's centre) where it is.
pub fn zoom_at(zoom: Zoom, at: Point, scale: f64, fitted: Size, view: Size) -> Zoom {
    zoom_at_with(zoom, at, scale, fitted, view, &ZoomConfig::default())
}

/// Zooms by multiplying the current scale by `factor` around `at`, using `config`.
pub fn zoom_by_factor_with(
    zoom: Zoom,
    at: Point,
    factor: f64,
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    if !factor.is_finite() {
        return clamp_with(zoom, fitted, view, config);
    }
    zoom_at_with(zoom, at, zoom.scale * factor, fitted, view, config)
}

/// Zooms by multiplying the current scale by `factor` around `at`.
pub fn zoom_by_factor(zoom: Zoom, at: Point, factor: f64, fitted: Size, view: Size) -> Zoom {
    zoom_by_factor_with(zoom, at, factor, fitted, view, &ZoomConfig::default())
}

/// The scale after a wheel turn of `delta_y` using `config` (positive is away from the person: zoom out).
pub fn wheel_scale_with(scale: f64, delta_y: f64, config: &ZoomConfig) -> f64 {
    let max_zoom = config.max_zoom.max(1.0);
    (scale * (-delta_y * config.wheel_step).exp()).clamp(1.0, max_zoom)
}

/// The scale after a wheel turn of `delta_y` (positive is away from the person: zoom out).
pub fn wheel_scale(scale: f64, delta_y: f64) -> f64 {
    wheel_scale_with(scale, delta_y, &ZoomConfig::default())
}

/// What a double tap zooms to using `config`: until the picture fills the view (a landscape picture
/// on a phone fills its height), at least `config.min_tap_zoom`.
pub fn tap_zoom_scale_with(fitted: Size, view: Size, config: &ZoomConfig) -> f64 {
    let max_zoom = config.max_zoom.max(1.0);
    let fill = (view.width / fitted.width.max(1.0)).max(view.height / fitted.height.max(1.0));
    fill.max(config.min_tap_zoom).min(max_zoom)
}

/// What a double tap zooms to: until the picture fills the view (a landscape picture on a phone
/// fills its height), at least `MIN_TAP_ZOOM`.
pub fn tap_zoom_scale(fitted: Size, view: Size) -> f64 {
    tap_zoom_scale_with(fitted, view, &ZoomConfig::default())
}

/// A double tap or double click using `config`: back to the whole picture when zoomed in, else in
/// on the spot.
pub fn toggle_zoom_with(
    zoom: Zoom,
    at: Point,
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    if zoom.is_zoomed_with(config) {
        Zoom::REST
    } else {
        zoom_at_with(
            zoom,
            at,
            tap_zoom_scale_with(fitted, view, config),
            fitted,
            view,
            config,
        )
    }
}

/// A double tap or double click: back to the whole picture when zoomed in, else in on the spot.
pub fn toggle_zoom(zoom: Zoom, at: Point, fitted: Size, view: Size) -> Zoom {
    toggle_zoom_with(zoom, at, fitted, view, &ZoomConfig::default())
}

/// Dragging a zoomed picture using `config`: the zoom when it started, moved by the finger or mouse.
pub fn pan_with(
    start: Zoom,
    dx: f64,
    dy: f64,
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    clamp_with(
        Zoom {
            x: start.x + dx,
            y: start.y + dy,
            ..start
        },
        fitted,
        view,
        config,
    )
}

/// Dragging a zoomed picture: the zoom when it started, moved by the finger or mouse.
pub fn pan(start: Zoom, dx: f64, dy: f64, fitted: Size, view: Size) -> Zoom {
    pan_with(start, dx, dy, fitted, view, &ZoomConfig::default())
}

/// What two fingers did since they went down using `config`: the zoom then, the midpoint and the
/// distance between the fingers then and now.
#[allow(clippy::too_many_arguments)]
pub fn pinch_with(
    start: Zoom,
    start_mid: Point,
    start_distance: f64,
    mid: Point,
    distance: f64,
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    if start_distance <= 0.0 || !distance.is_finite() {
        return start;
    }
    let max_zoom = config.max_zoom.max(1.0);
    let scale = (start.scale * distance / start_distance).clamp(1.0, max_zoom);
    let k = scale / start.scale;
    clamp_with(
        Zoom {
            scale,
            x: mid.x - (start_mid.x - start.x) * k,
            y: mid.y - (start_mid.y - start.y) * k,
        },
        fitted,
        view,
        config,
    )
}

/// What two fingers did since they went down: the zoom then, the midpoint and the distance between
/// the fingers then and now. The picture follows the midpoint and scales with the distance.
pub fn pinch(
    start: Zoom,
    start_mid: Point,
    start_distance: f64,
    mid: Point,
    distance: f64,
    fitted: Size,
    view: Size,
) -> Zoom {
    pinch_with(
        start,
        start_mid,
        start_distance,
        mid,
        distance,
        fitted,
        view,
        &ZoomConfig::default(),
    )
}

fn midpoint(a: Point, b: Point) -> Point {
    Point {
        x: (a.x + b.x) / 2.0,
        y: (a.y + b.y) / 2.0,
    }
}

fn distance(a: Point, b: Point) -> f64 {
    (b.x - a.x).hypot(b.y - a.y)
}

/// Pinch computed directly from the two start points and two current points using `config`.
pub fn pinch_points_with(
    start: Zoom,
    start_points: (Point, Point),
    points: (Point, Point),
    fitted: Size,
    view: Size,
    config: &ZoomConfig,
) -> Zoom {
    pinch_with(
        start,
        midpoint(start_points.0, start_points.1),
        distance(start_points.0, start_points.1),
        midpoint(points.0, points.1),
        distance(points.0, points.1),
        fitted,
        view,
        config,
    )
}

/// Pinch computed directly from the two start points and two current points.
pub fn pinch_points(
    start: Zoom,
    start_points: (Point, Point),
    points: (Point, Point),
    fitted: Size,
    view: Size,
) -> Zoom {
    pinch_points_with(
        start,
        start_points,
        points,
        fitted,
        view,
        &ZoomConfig::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEW: Size = Size {
        width: 400.0,
        height: 800.0,
    };
    const FITTED: Size = Size {
        width: 400.0,
        height: 300.0,
    };

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn a_picture_fits_inside_the_view_keeping_its_shape() {
        let fitted = fit_within(
            Size {
                width: 4000.0,
                height: 3000.0,
            },
            VIEW,
        )
        .unwrap();
        assert_eq!(
            fitted,
            Size {
                width: 400.0,
                height: 300.0
            }
        );
        let tall = fit_within(
            Size {
                width: 1000.0,
                height: 4000.0,
            },
            VIEW,
        )
        .unwrap();
        assert_eq!(
            tall,
            Size {
                width: 200.0,
                height: 800.0
            }
        );
        assert_eq!(
            fit_within(
                Size {
                    width: 0.0,
                    height: 10.0
                },
                VIEW
            ),
            None
        );
        assert_eq!(
            fit_within(
                Size {
                    width: 10.0,
                    height: 10.0
                },
                Size {
                    width: 0.0,
                    height: 5.0
                }
            ),
            None
        );
    }

    #[test]
    fn the_scale_stays_between_one_and_the_most() {
        assert_eq!(
            clamp(
                Zoom {
                    scale: 0.2,
                    x: 50.0,
                    y: 50.0
                },
                FITTED,
                VIEW
            ),
            Zoom::REST
        );
        assert_eq!(
            clamp(
                Zoom {
                    scale: 40.0,
                    x: 0.0,
                    y: 0.0
                },
                FITTED,
                VIEW
            )
            .scale,
            MAX_ZOOM
        );
        assert_eq!(
            clamp(
                Zoom {
                    scale: f64::NAN,
                    x: f64::NAN,
                    y: 1.0
                },
                FITTED,
                VIEW
            ),
            Zoom::REST
        );
    }

    #[test]
    fn panning_stops_at_the_pictures_edges() {
        // At 2x the picture is 800 wide in a 400 view: it may shift 200 each way. It is 600 tall in
        // an 800 view: it stays centred vertically.
        let z = pan(
            Zoom {
                scale: 2.0,
                x: 0.0,
                y: 0.0,
            },
            500.0,
            500.0,
            FITTED,
            VIEW,
        );
        assert!(near(z.x, 200.0));
        assert!(near(z.y, 0.0));
        let z = pan(
            Zoom {
                scale: 6.0,
                x: 0.0,
                y: 0.0,
            },
            -9999.0,
            -9999.0,
            FITTED,
            VIEW,
        );
        assert!(near(z.x, -(400.0 * 6.0 - 400.0) / 2.0));
        assert!(near(z.y, -(300.0 * 6.0 - 800.0) / 2.0));
    }

    #[test]
    fn zooming_keeps_the_point_under_the_pointer() {
        // A point 100px right of the centre, zooming 1x to 2x: the picture point under it must stay.
        let at = Point { x: 100.0, y: 0.0 };
        let z = zoom_at(Zoom::REST, at, 2.0, FITTED, VIEW);
        // Before: the picture point is 100 from the centre. After: 100 / 2... it sits at
        // x + 2 * 100 = 100 again only if x = -100.
        assert!(near(z.x, -100.0));
        // Zooming out to 1x brings the picture back to the centre.
        assert_eq!(zoom_at(z, at, 1.0, FITTED, VIEW), Zoom::REST);
    }

    #[test]
    fn the_wheel_zooms_in_and_out_within_bounds() {
        assert!(wheel_scale(1.0, -100.0) > 1.2);
        assert!(near(wheel_scale(1.0, 100.0), 1.0));
        assert!(near(wheel_scale(5.9, -1000.0), MAX_ZOOM));
        assert!(wheel_scale(2.0, 100.0) < 2.0);
    }

    #[test]
    fn a_double_tap_zooms_to_fill_or_the_minimum_and_back() {
        // 400x300 in 400x800: filling the height takes 800/300 = 2.67, below the minimum.
        assert!(near(tap_zoom_scale(FITTED, VIEW), MIN_TAP_ZOOM));
        // A wide strip in a tall view fills at 6 at most.
        let strip = Size {
            width: 400.0,
            height: 50.0,
        };
        assert!(near(tap_zoom_scale(strip, VIEW), MAX_ZOOM));
        let zoomed = toggle_zoom(Zoom::REST, Point { x: 0.0, y: 0.0 }, FITTED, VIEW);
        assert!(zoomed.is_zoomed());
        assert_eq!(
            toggle_zoom(zoomed, Point { x: 5.0, y: 5.0 }, FITTED, VIEW),
            Zoom::REST
        );
    }

    #[test]
    fn a_pinch_scales_with_the_fingers_and_follows_their_midpoint() {
        let mid = Point { x: 0.0, y: 0.0 };
        let z = pinch(Zoom::REST, mid, 100.0, mid, 200.0, FITTED, VIEW);
        assert!(near(z.scale, 2.0));
        assert!(near(z.x, 0.0));
        // Fingers moving right by 50 with the same spread pan a zoomed picture right.
        let z = pinch(
            Zoom {
                scale: 2.0,
                x: 0.0,
                y: 0.0,
            },
            mid,
            100.0,
            Point { x: 50.0, y: 0.0 },
            100.0,
            FITTED,
            VIEW,
        );
        assert!(near(z.x, 50.0));
        // Pinching in below 1x stops at the whole picture.
        assert_eq!(
            pinch(Zoom::REST, mid, 100.0, mid, 10.0, FITTED, VIEW),
            Zoom::REST
        );
        // No distance to compare with: nothing changes.
        assert_eq!(
            pinch(Zoom::REST, mid, 0.0, mid, 50.0, FITTED, VIEW),
            Zoom::REST
        );
    }

    #[test]
    fn zoom_config_overrides_defaults() {
        let config = ZoomConfig {
            max_zoom: 4.0,
            min_tap_zoom: 2.0,
            zoomed_above: 1.05,
            wheel_step: 0.01,
        };
        assert!(
            !Zoom {
                scale: 1.03,
                x: 0.0,
                y: 0.0
            }
            .is_zoomed_with(&config)
        );
        assert!(
            Zoom {
                scale: 1.06,
                x: 0.0,
                y: 0.0
            }
            .is_zoomed_with(&config)
        );
        assert_eq!(
            clamp_with(
                Zoom {
                    scale: 10.0,
                    x: 0.0,
                    y: 0.0
                },
                FITTED,
                VIEW,
                &config
            )
            .scale,
            4.0
        );
        assert!(near(tap_zoom_scale_with(FITTED, VIEW, &config), 2.6666667));
        let strip = Size {
            width: 400.0,
            height: 50.0,
        };
        assert!(near(tap_zoom_scale_with(strip, VIEW, &config), 4.0));
        assert!(near(wheel_scale_with(1.0, -100.0, &config), 1.0_f64.exp()));
        let toggled = toggle_zoom_with(Zoom::REST, Point { x: 0.0, y: 0.0 }, FITTED, VIEW, &config);
        assert!(toggled.is_zoomed_with(&config));
        assert_eq!(
            toggle_zoom_with(toggled, Point { x: 0.0, y: 0.0 }, FITTED, VIEW, &config),
            Zoom::REST
        );
    }

    #[test]
    fn zoom_by_factor_scales_around_the_pointer() {
        let at = Point { x: 100.0, y: 0.0 };
        let z = zoom_by_factor(Zoom::REST, at, 2.0, FITTED, VIEW);
        assert!(near(z.scale, 2.0));
        assert!(near(z.x, -100.0));
        let z2 = zoom_by_factor(z, at, 0.5, FITTED, VIEW);
        assert_eq!(z2, Zoom::REST);
        let config = ZoomConfig {
            max_zoom: 3.0,
            ..ZoomConfig::default()
        };
        let capped = zoom_by_factor_with(z, at, 4.0, FITTED, VIEW, &config);
        assert!(near(capped.scale, 3.0));
    }

    #[test]
    fn pinch_points_computes_midpoint_and_distance() {
        let start_a = Point { x: -50.0, y: 0.0 };
        let start_b = Point { x: 50.0, y: 0.0 };
        let cur_a = Point { x: -100.0, y: 0.0 };
        let cur_b = Point { x: 100.0, y: 0.0 };
        let z = pinch_points(Zoom::REST, (start_a, start_b), (cur_a, cur_b), FITTED, VIEW);
        assert!(near(z.scale, 2.0));
        assert!(near(z.x, 0.0));

        // Shifting both points right by 50 at the same spread pans right by 50.
        let shifted = pinch_points(
            Zoom {
                scale: 2.0,
                x: 0.0,
                y: 0.0,
            },
            (start_a, start_b),
            (Point { x: 0.0, y: 0.0 }, Point { x: 100.0, y: 0.0 }),
            FITTED,
            VIEW,
        );
        assert!(near(shifted.scale, 2.0));
        assert!(near(shifted.x, 50.0));

        // Zero initial distance is a no-op.
        assert_eq!(
            pinch_points(Zoom::REST, (start_a, start_a), (cur_a, cur_b), FITTED, VIEW),
            Zoom::REST
        );
    }
}
