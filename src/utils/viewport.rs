//! Where a pannable, zoomable surface is looking, and the arithmetic to move it.
//!
//! One convention, and everything else follows from it: **screen = world × zoom + offset**. The
//! offset is where the world's origin lands on screen, in screen pixels, so panning is addition
//! and never has to know the zoom.
//!
//! Deliberately free of the DOM. `window()` *panics* off-wasm rather than returning zero, so
//! anything that reads the viewport's size directly cannot be tested at all — the same reason
//! [`crate::primitives::floating_menu`]'s clamp is a pure function. What is here is arithmetic
//! with unit tests; the primitive reads the boxes and hands them in.
//!
//! The piece worth having tests for is [`Viewport::zoomed_to`]. Zooming toward the middle of the
//! surface is one line and wrong: the thing under the pointer has to stay under the pointer, or
//! every scroll of the wheel slides the canvas out from under whatever you were looking at.

/// How far a surface may be zoomed. A canvas that can reach 0 is a canvas you cannot get back.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ZoomLimits {
    pub min: f64,
    pub max: f64,
}

impl Default for ZoomLimits {
    fn default() -> Self {
        Self { min: 0.1, max: 4.0 }
    }
}

impl ZoomLimits {
    pub fn clamp(&self, zoom: f64) -> f64 {
        // `min` last, so a NaN or an inverted pair still lands somewhere usable rather than
        // leaving the surface at a zoom nothing can be drawn at.
        zoom.min(self.max).max(self.min)
    }
}

/// One of the eight grips on a box: four corners that move two edges, four sides that move one.
///
/// Its own enum rather than [`crate::utils::image::CropHandle`], which looks the same and is not:
/// a crop is normalised to its image and clamped to it, and a node on a canvas is neither. What
/// they share is the eight names, which is not enough to share a type over.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoxHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl BoxHandle {
    /// Every handle, clockwise from the top left — the order they are drawn in.
    pub const ALL: [BoxHandle; 8] = [
        BoxHandle::TopLeft,
        BoxHandle::Top,
        BoxHandle::TopRight,
        BoxHandle::Right,
        BoxHandle::BottomRight,
        BoxHandle::Bottom,
        BoxHandle::BottomLeft,
        BoxHandle::Left,
    ];

    /// The four that move two edges at once, and the only ones worth drawing when a ratio has to
    /// be kept: a side on its own cannot honour one without moving an edge the pointer is not on.
    pub const CORNERS: [BoxHandle; 4] = [
        BoxHandle::TopLeft,
        BoxHandle::TopRight,
        BoxHandle::BottomRight,
        BoxHandle::BottomLeft,
    ];

    /// Written into `data-handle`, and what a cursor is chosen by.
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxHandle::TopLeft => "top-left",
            BoxHandle::Top => "top",
            BoxHandle::TopRight => "top-right",
            BoxHandle::Right => "right",
            BoxHandle::BottomRight => "bottom-right",
            BoxHandle::Bottom => "bottom",
            BoxHandle::BottomLeft => "bottom-left",
            BoxHandle::Left => "left",
        }
    }

    pub fn is_corner(&self) -> bool {
        Self::CORNERS.contains(self)
    }

    pub fn moves_left(&self) -> bool {
        matches!(
            self,
            BoxHandle::TopLeft | BoxHandle::Left | BoxHandle::BottomLeft
        )
    }

    pub fn moves_right(&self) -> bool {
        matches!(
            self,
            BoxHandle::TopRight | BoxHandle::Right | BoxHandle::BottomRight
        )
    }

    pub fn moves_top(&self) -> bool {
        matches!(
            self,
            BoxHandle::TopLeft | BoxHandle::Top | BoxHandle::TopRight
        )
    }

    pub fn moves_bottom(&self) -> bool {
        matches!(
            self,
            BoxHandle::BottomLeft | BoxHandle::Bottom | BoxHandle::BottomRight
        )
    }

    /// Where the grip sits on the box, as a fraction of its width and height. What the primitive
    /// places it by, so the eight positions are written down once rather than in a stylesheet.
    pub fn fractions(&self) -> (f64, f64) {
        let x = match (self.moves_left(), self.moves_right()) {
            (true, _) => 0.0,
            (_, true) => 1.0,
            _ => 0.5,
        };
        let y = match (self.moves_top(), self.moves_bottom()) {
            (true, _) => 0.0,
            (_, true) => 1.0,
            _ => 0.5,
        };
        (x, y)
    }
}

/// A box in world coordinates — what [`Viewport::fitted`] is asked to bring into view, and what a
/// node on a canvas is.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WorldBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// Rotation about the box's own centre, in **radians**, clockwise on screen. `x`, `y`,
    /// `width` and `height` describe the box *before* it is turned, so everything that reads them
    /// is unaffected by an angle it does not know about.
    ///
    /// **Reserved, and deliberately ahead of the gestures that would set it.** A node carries it,
    /// renders it and reports it through `CanvasGesture`, so it is already in the shape a board
    /// gets saved and sent in — but nothing drags it yet, and the two things that assume an
    /// axis-aligned box say so where they are: [`WorldBox::resized`] and [`WorldBox::around`].
    /// The cost of adding this later was never the code, it was migrating every board that had
    /// been made in the meantime, and that part is paid.
    ///
    /// Radians rather than degrees because everything that will read it is trigonometry, and a
    /// unit that has to be converted at every use is a unit that eventually is not.
    pub angle: f64,
}

impl WorldBox {
    /// An upright box. The angle is its own call — see [`WorldBox::rotated`] — so that adding one
    /// left every caller of this alone.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            angle: 0.0,
        }
    }

    /// The same box, turned about its centre.
    pub fn rotated(self, angle: f64) -> Self {
        Self { angle, ..self }
    }

    /// Whether it is upright, which is the case every piece of arithmetic here is written for.
    pub fn is_upright(&self) -> bool {
        self.angle == 0.0
    }

    /// The smallest box containing all of them, or `None` for nothing at all — which is a real
    /// answer rather than an empty box at the origin, since "fit nothing" has no meaning.
    ///
    /// Each one is taken as upright, so the result under-covers a rotated box by however far its
    /// corners swing outside it. Projecting those four corners first is part of the rotation work
    /// that is deferred; until something can be rotated by hand, nothing reaches this that is.
    /// The result is upright whatever went in, which is what a bounding box is.
    pub fn around(boxes: impl IntoIterator<Item = WorldBox>) -> Option<Self> {
        let mut boxes = boxes.into_iter();
        let first = boxes.next()?;

        let (mut left, mut top) = (first.x, first.y);
        let (mut right, mut bottom) = (first.x + first.width, first.y + first.height);

        for item in boxes {
            left = left.min(item.x);
            top = top.min(item.y);
            right = right.max(item.x + item.width);
            bottom = bottom.max(item.y + item.height);
        }

        Some(Self::new(left, top, right - left, bottom - top))
    }

    /// The box a resize leaves behind, given the distance the pointer has travelled **since the
    /// press**, in world units.
    ///
    /// Measured from the press rather than from the last move, because a resize is a box computed
    /// from one number rather than a box nudged repeatedly: clamping an incremental version at
    /// `min` throws the remainder away, and the box then lags the pointer by however far it was
    /// dragged past the limit.
    ///
    /// The edges the handle is not on do not move — that is what makes it feel like dragging a
    /// corner rather than moving the whole thing — and a drag past the far edge stops at `min`
    /// instead of turning the box inside out.
    ///
    /// **Correct only for an upright box.** `dx` and `dy` arrive in world axes and a rotated box's
    /// edges do not lie along them, so a rotated one would grow in the wrong direction; the angle
    /// is carried through untouched and `CanvasNodeRoot` draws no grips on a node that has one.
    /// Rotating the delta into the box's own frame before this, and back after, is the deferred
    /// half of the work.
    ///
    /// `aspect` is width ÷ height. With one, a corner is driven by its horizontal travel and the
    /// height follows; a side handle drives whichever axis it moves and grows the other about the
    /// middle, since it has no opposite edge on that axis to hold still.
    pub fn resized(
        self,
        handle: BoxHandle,
        dx: f64,
        dy: f64,
        min: f64,
        aspect: Option<f64>,
    ) -> Self {
        let min = min.max(0.0);
        let (mut left, mut top) = (self.x, self.y);
        let (mut right, mut bottom) = (self.x + self.width, self.y + self.height);

        if handle.moves_left() {
            left = (left + dx).min(right - min);
        }
        if handle.moves_right() {
            right = (right + dx).max(left + min);
        }
        if handle.moves_top() {
            top = (top + dy).min(bottom - min);
        }
        if handle.moves_bottom() {
            bottom = (bottom + dy).max(top + min);
        }

        let Some(aspect) = aspect.filter(|aspect| aspect.is_finite() && *aspect > 0.0) else {
            return Self::new(left, top, right - left, bottom - top).rotated(self.angle);
        };

        // One axis is chosen and the other follows it. A corner has moved both, so the width wins
        // and the height is derived; a side has moved only one, and that is the one to follow.
        let (width, height) = if handle.moves_left() || handle.moves_right() {
            let width = (right - left).max(min);
            (width, width / aspect)
        } else {
            let height = (bottom - top).max(min);
            (height * aspect, height)
        };

        // The unmoved axis keeps its middle, which is the only fixed point a side handle leaves.
        let (left, right) = match (handle.moves_left(), handle.moves_right()) {
            (true, _) => (right - width, right),
            (_, true) => (left, left + width),
            _ => {
                let middle = (left + right) / 2.0;
                (middle - width / 2.0, middle + width / 2.0)
            }
        };
        let (top, bottom) = match (handle.moves_top(), handle.moves_bottom()) {
            (true, _) => (bottom - height, bottom),
            (_, true) => (top, top + height),
            _ => {
                let middle = (top + bottom) / 2.0;
                (middle - height / 2.0, middle + height / 2.0)
            }
        };

        Self::new(left, top, right - left, bottom - top).rotated(self.angle)
    }
}

/// A pan offset in screen pixels and a zoom factor.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Viewport {
    /// Where the world's origin sits on screen.
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    pub fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        (x * self.zoom + self.x, y * self.zoom + self.y)
    }

    pub fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        if self.zoom == 0.0 {
            return (0.0, 0.0);
        }
        ((x - self.x) / self.zoom, (y - self.y) / self.zoom)
    }

    /// Moves the surface by a screen-pixel delta. Panning is the same distance whatever the zoom,
    /// because a drag is measured in the pixels the pointer actually travelled.
    pub fn panned(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            zoom: self.zoom,
        }
    }

    /// Zooms to `zoom`, keeping whatever is under `(at_x, at_y)` under it.
    ///
    /// The anchor is found *before* the zoom changes and the offset re-derived from it after, so
    /// clamping cannot break the promise: a zoom refused at the limit leaves the surface exactly
    /// where it was rather than sliding it by the difference.
    pub fn zoomed_to(&self, zoom: f64, at_x: f64, at_y: f64, limits: ZoomLimits) -> Self {
        let (world_x, world_y) = self.screen_to_world(at_x, at_y);
        let zoom = limits.clamp(zoom);

        Self {
            x: at_x - world_x * zoom,
            y: at_y - world_y * zoom,
            zoom,
        }
    }

    /// Multiplies the zoom, which is what a wheel notch means — two notches in should be the same
    /// however far in you already are, and that is a ratio rather than an amount.
    pub fn zoomed_by(&self, factor: f64, at_x: f64, at_y: f64, limits: ZoomLimits) -> Self {
        self.zoomed_to(self.zoom * factor, at_x, at_y, limits)
    }

    /// Brings `content` into a surface `width` × `height`, with `padding` screen pixels to spare.
    ///
    /// Content with no extent is not an error and not a division: a single point still has a
    /// place to sit, so the zoom is left alone and the point is centred.
    pub fn fitted(
        content: WorldBox,
        width: f64,
        height: f64,
        padding: f64,
        limits: ZoomLimits,
    ) -> Self {
        let usable_width = (width - padding * 2.0).max(1.0);
        let usable_height = (height - padding * 2.0).max(1.0);

        let zoom = if content.width <= 0.0 || content.height <= 0.0 {
            1.0
        } else {
            limits.clamp((usable_width / content.width).min(usable_height / content.height))
        };

        let centre_x = content.x + content.width / 2.0;
        let centre_y = content.y + content.height / 2.0;

        Self {
            x: width / 2.0 - centre_x * zoom,
            y: height / 2.0 - centre_y * zoom,
            zoom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn screen_and_world_are_inverses() {
        let viewport = Viewport {
            x: 130.0,
            y: -40.0,
            zoom: 2.5,
        };

        let (world_x, world_y) = viewport.screen_to_world(17.0, 92.0);
        let (screen_x, screen_y) = viewport.world_to_screen(world_x, world_y);

        assert!(close(screen_x, 17.0) && close(screen_y, 92.0));
    }

    #[test]
    fn panning_does_not_depend_on_zoom() {
        let a = Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 0.25,
        }
        .panned(30.0, -10.0);
        let b = Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 4.0,
        }
        .panned(30.0, -10.0);

        assert_eq!((a.x, a.y), (b.x, b.y));
    }

    #[test]
    fn zooming_keeps_the_point_under_the_pointer() {
        let viewport = Viewport {
            x: 12.0,
            y: -7.0,
            zoom: 1.0,
        };
        let (pointer_x, pointer_y) = (300.0, 220.0);
        let before = viewport.screen_to_world(pointer_x, pointer_y);

        let zoomed = viewport.zoomed_by(2.0, pointer_x, pointer_y, ZoomLimits::default());
        let after = zoomed.screen_to_world(pointer_x, pointer_y);

        assert!(close(before.0, after.0) && close(before.1, after.1));
    }

    #[test]
    fn a_zoom_refused_at_the_limit_does_not_move_the_surface() {
        let limits = ZoomLimits::default();
        let at_the_top = Viewport {
            x: 55.0,
            y: 5.0,
            zoom: limits.max,
        };

        let refused = at_the_top.zoomed_by(2.0, 400.0, 300.0, limits);

        assert_eq!(refused, at_the_top);
    }

    #[test]
    fn zoom_is_clamped_both_ways() {
        let limits = ZoomLimits { min: 0.5, max: 2.0 };
        let viewport = Viewport::default();

        assert_eq!(viewport.zoomed_by(100.0, 0.0, 0.0, limits).zoom, 2.0);
        assert_eq!(viewport.zoomed_by(0.001, 0.0, 0.0, limits).zoom, 0.5);
    }

    #[test]
    fn fitting_centres_the_content() {
        let content = WorldBox::new(-50.0, -50.0, 100.0, 100.0);
        let viewport = Viewport::fitted(content, 400.0, 400.0, 0.0, ZoomLimits::default());

        let (screen_x, screen_y) = viewport.world_to_screen(0.0, 0.0);
        assert!(close(screen_x, 200.0) && close(screen_y, 200.0));
        assert!(close(viewport.zoom, 4.0));
    }

    #[test]
    fn fitting_uses_the_tighter_axis_and_honours_padding() {
        // Wide content in a square surface: the width is what runs out first.
        let content = WorldBox::new(0.0, 0.0, 200.0, 50.0);
        let viewport = Viewport::fitted(content, 400.0, 400.0, 20.0, ZoomLimits::default());

        assert!(close(viewport.zoom, 360.0 / 200.0));
    }

    #[test]
    fn fitting_a_point_leaves_the_zoom_alone() {
        let point = WorldBox::new(10.0, 10.0, 0.0, 0.0);
        let viewport = Viewport::fitted(point, 400.0, 300.0, 24.0, ZoomLimits::default());

        assert_eq!(viewport.zoom, 1.0);
        let (screen_x, screen_y) = viewport.world_to_screen(10.0, 10.0);
        assert!(close(screen_x, 200.0) && close(screen_y, 150.0));
    }

    #[test]
    fn a_corner_moves_two_edges_and_leaves_the_far_one_alone() {
        let box_ = WorldBox::new(10.0, 20.0, 100.0, 50.0);
        let resized = box_.resized(BoxHandle::TopLeft, -10.0, -5.0, 8.0, None);

        assert_eq!(resized, WorldBox::new(0.0, 15.0, 110.0, 55.0));
        // The bottom right is where it was.
        assert!(close(resized.x + resized.width, 110.0));
        assert!(close(resized.y + resized.height, 70.0));
    }

    #[test]
    fn a_side_moves_one_edge_only() {
        let box_ = WorldBox::new(0.0, 0.0, 100.0, 40.0);
        let resized = box_.resized(BoxHandle::Right, 25.0, 999.0, 8.0, None);

        assert_eq!(resized, WorldBox::new(0.0, 0.0, 125.0, 40.0));
    }

    #[test]
    fn dragging_past_the_far_edge_stops_at_the_minimum() {
        let box_ = WorldBox::new(0.0, 0.0, 100.0, 100.0);
        let resized = box_.resized(BoxHandle::Right, -500.0, 0.0, 20.0, None);

        assert_eq!(resized, WorldBox::new(0.0, 0.0, 20.0, 100.0));
        // And never inside out, whichever handle is doing it.
        let flipped = box_.resized(BoxHandle::TopLeft, 500.0, 500.0, 20.0, None);
        assert!(flipped.width >= 20.0 && flipped.height >= 20.0);
    }

    #[test]
    fn a_ratio_is_kept_from_the_corner_being_dragged() {
        let square = WorldBox::new(0.0, 0.0, 100.0, 100.0);
        let resized = square.resized(BoxHandle::BottomRight, 40.0, 0.0, 8.0, Some(1.0));

        // The pointer moved on x alone; the height came with it, and the top left held.
        assert_eq!(resized, WorldBox::new(0.0, 0.0, 140.0, 140.0));
    }

    #[test]
    fn a_side_with_a_ratio_grows_the_other_axis_about_the_middle() {
        let square = WorldBox::new(0.0, 0.0, 100.0, 100.0);
        let resized = square.resized(BoxHandle::Right, 100.0, 0.0, 8.0, Some(1.0));

        assert!(close(resized.width, 200.0) && close(resized.height, 200.0));
        // The vertical middle is where it was: 50.
        assert!(close(resized.y + resized.height / 2.0, 50.0));
    }

    #[test]
    fn a_resize_measured_from_the_press_does_not_lag_past_the_limit() {
        let box_ = WorldBox::new(0.0, 0.0, 100.0, 100.0);
        // Dragged well past the minimum, then most of the way back. Cumulative travel is what
        // decides, so the box is where the pointer is rather than where the clamping left it.
        let over = box_.resized(BoxHandle::Right, -300.0, 0.0, 20.0, None);
        let back = box_.resized(BoxHandle::Right, -40.0, 0.0, 20.0, None);

        assert_eq!(over.width, 20.0);
        assert_eq!(back.width, 60.0);
    }

    #[test]
    fn every_handle_has_its_own_corner_of_the_box() {
        let fractions: Vec<(f64, f64)> = BoxHandle::ALL.iter().map(|h| h.fractions()).collect();
        let mut unique = fractions.clone();
        unique.dedup();

        assert_eq!(fractions.len(), 8);
        assert_eq!(unique.len(), 8);
        assert_eq!(BoxHandle::TopLeft.fractions(), (0.0, 0.0));
        assert_eq!(BoxHandle::BottomRight.fractions(), (1.0, 1.0));
        assert_eq!(BoxHandle::Right.fractions(), (1.0, 0.5));
    }

    #[test]
    fn a_box_is_upright_unless_it_is_told_otherwise() {
        let upright = WorldBox::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(upright.angle, 0.0);
        assert!(upright.is_upright());
        assert!(!upright.rotated(0.5).is_upright());
        // And the turn changes nothing else, since the four numbers describe the box before it.
        assert_eq!(
            WorldBox {
                angle: 0.0,
                ..upright.rotated(0.5)
            },
            upright
        );
    }

    #[test]
    fn a_resize_carries_the_angle_through_untouched() {
        let tilted = WorldBox::new(0.0, 0.0, 100.0, 100.0).rotated(0.5);

        assert_eq!(
            tilted.resized(BoxHandle::Right, 20.0, 0.0, 8.0, None).angle,
            0.5
        );
        assert_eq!(
            tilted
                .resized(BoxHandle::BottomRight, 20.0, 0.0, 8.0, Some(1.0))
                .angle,
            0.5
        );
    }

    #[test]
    fn a_bounding_box_is_upright_whatever_went_into_it() {
        let around = WorldBox::around([
            WorldBox::new(0.0, 0.0, 10.0, 10.0).rotated(1.0),
            WorldBox::new(20.0, 0.0, 10.0, 10.0).rotated(-1.0),
        ])
        .unwrap();

        assert!(around.is_upright());
    }

    #[test]
    fn a_box_around_nothing_is_nothing() {
        assert_eq!(WorldBox::around([]), None);
    }

    #[test]
    fn a_box_around_several_covers_them_all() {
        let around = WorldBox::around([
            WorldBox::new(0.0, 0.0, 10.0, 10.0),
            WorldBox::new(-5.0, 20.0, 10.0, 10.0),
        ])
        .unwrap();

        assert_eq!(around, WorldBox::new(-5.0, 0.0, 15.0, 30.0));
    }
}
